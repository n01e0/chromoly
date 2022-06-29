use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

pub(crate) fn chrome_cookies() -> Result<Vec<PathBuf>> {
    let config_home =
        env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", env::var("HOME")?));

    let chrome_config = format!("{}/google-chrome", config_home);

    Ok(WalkDir::new(chrome_config)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file())
        .filter(|f| {
            f.path()
                .to_str()
                .with_context(|| format!("Can't convert path to str"))
                .unwrap()
                .contains("Cookies")
        })
        .map(|f| f.into_path())
        .collect::<Vec<_>>())
}

#[test]
fn cookie_found() {
    color_backtrace::install();
    let cookies = chrome_cookies().unwrap();
    assert!(cookies.len() > 0);
}
