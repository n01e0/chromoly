use crate::config::Config;
use anyhow::Result;
use std::path::{Path, PathBuf};
use which::which;

pub(crate) fn chrome_binary(cfg: &Config) -> Result<Vec<PathBuf>> {
    if let Some(bin) = cfg.get_chrome_binary() {
        return Ok(bin
            .into_iter()
            .map(|p| Path::new(p).to_path_buf())
            .collect());
    }
    Ok(vec![
        which("google-chrome")?,
        which("google-chrome-stable")?,
        Path::new("/opt/google/chrome/chrome").to_path_buf(),
    ])
}
