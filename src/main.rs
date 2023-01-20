mod cli;
mod rule;
mod config;
mod proxy;

use clap::Parser;
use wirefilter::Type;

fn main() {
    let args = cli::Args::parse();
    println!("{:?}", args);
}
