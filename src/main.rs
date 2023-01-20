mod config;
mod proxy;
mod rule;

use anyhow::{anyhow, Result};
use clap::Parser;
use wirefilter::Type;

use crate::config::Config;

fn main() -> Result<()> {
    let args = Args::parse();

    let path = args.config;
    let config = match std::fs::read_to_string(&path) {
        Ok(c) => Config::new(&c),
        _ => Err(anyhow!("no such file '{}'", path)),
    }?;

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(short, long)]
    pub config: String,
}
