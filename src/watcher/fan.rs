use super::filter::chrome_binary;
use crate::{config::Config, daemon::daemonize, profile};
use anyhow::{Context, Result};
use fanotify::high_level::*;
use log::{error, info, warn};
use nix::poll::{poll, PollFd, PollFlags};
use procfs::process::Process;

pub fn watch(cfg: &Config) -> Result<()> {
    info!("initializing fanotify...");
    let fan = Fanotify::new_with_nonblocking(FanotifyMode::PRECONTENT);

    if let Some(cookies) = cfg.get_cookies() {
        for cookie in cookies {
            info!("add {} to target", cookie);
            fan.add_path(FAN_OPEN_PERM, cookie)?;
        }
    } else {
        for c in profile::chrome_cookies().with_context(|| "Can't found chrome cookies")? {
            info!("add {} to target", c.display());
            fan.add_path(FAN_OPEN_PERM, c.as_path())?;
        }
    }

    let mut fds = [PollFd::new(fan.as_raw_fd(), PollFlags::POLLIN)];

    if cfg.daemonize.unwrap_or(true) {
        daemonize(cfg)?;
    }

    let chromes = chrome_binary(&cfg)?;

    loop {
        let poll_num = poll(&mut fds, -1).with_context(|| {
            error!("Can't poll");
            "Can't poll"
        })?;

        if poll_num > 0 {
            for event in fan.read_event() {
                let mut resp = FanotifyResponse::Allow;
                match Process::new(event.pid as i32) {
                    Ok(proc) => {
                        if let Ok(exe) = proc.exe() {
                            if !chromes.contains(&exe) {
                                warn!(
                                    "{} is probably not in chrome, so I blocked it.",
                                    exe.as_path().display()
                                );
                                resp = FanotifyResponse::Deny;
                            } else {
                                info!("{} has accessed cookies", exe.as_path().display());
                            }
                        } else {
                            warn!("Unknown process tried to access Cookies.");
                        }
                    }
                    Err(e) => {
                        error!("Can't stat proc info {}, block that.", e);
                        resp = FanotifyResponse::Deny;
                    }
                }

                fan.send_response(event.fd, resp);
            }
        } else {
            error!("wtf");
        }
    }
}
