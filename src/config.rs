use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs::read_to_string;
use tia::Tia;

#[derive(Parser, Debug, Tia)]
#[clap(author, version, about, long_about = None)]
#[tia(rg)]
pub struct Args {
    /// config file
    #[clap(short, long)]
    config: String,
    /// not daemonize
    #[clap(long, default_value_t = false, action)]
    pub(self) no_daemonize: bool,
}

#[derive(Debug, Tia, Deserialize)]
#[tia(rg)]
pub struct Config {
    pid_file: String,
    log_file: String,
    user: String,
    workdir: String,
    chrome_binary: Option<Vec<String>>,
    cookies: Option<Vec<String>>,
    pub(crate) daemonize: Option<bool>,
}

impl Config {
    pub fn from_args(args: Args) -> Result<Self> {
        let mut cfg: Config = serde_yaml::from_str(
            &read_to_string(args.get_config())
                .with_context(|| "Are you passing the correct config file?")?,
        )
        .with_context(|| format!("Can't parse config from {}", args.get_config()))?;

        cfg.daemonize = Some(!args.no_daemonize);

        Ok(cfg)
    }
}
