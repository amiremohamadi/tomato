use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(short, long)]
    config: Option<String>,
}
