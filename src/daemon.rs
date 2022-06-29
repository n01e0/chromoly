use anyhow::{Context, Result};
use daemonize::Daemonize;
use log::info;
use std::fs::OpenOptions;

use crate::config::Config;

fn exit_action() {
    info!("daemon spawned")
}

pub fn daemonize(cfg: &Config) -> Result<()> {
    info!("Starting chromoly daemon...");

    let stderr = OpenOptions::new()
        .write(true)
        .create(true)
        .open(cfg.get_log_file())
        .with_context(|| format!("failed to open or create {}", cfg.get_log_file()))?;

    Daemonize::new()
        .pid_file(cfg.get_pid_file())
        .user(&cfg.get_user()[..])
        .stderr(stderr)
        .working_directory(cfg.get_workdir())
        .exit_action(exit_action)
        .start()
        .with_context(|| "daemonize failed")
}
