use anyhow::Result;
use clap::Parser;
use std::env;

mod config;
mod daemon;
mod profile;
mod watcher;

fn main() -> Result<()> {
    let cmd = config::Args::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO")
    }

    env_logger::init();

    let cfg = config::Config::from_args(cmd)?;
    watcher::fan::watch(&cfg)?;

    Ok(())
}
