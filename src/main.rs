mod cli;
mod config;
mod proxy;
mod rule;

use clap::Parser;
use wirefilter::Type;

fn main() {
    let args = cli::Args::parse();
    println!("{:?}", args);
}
