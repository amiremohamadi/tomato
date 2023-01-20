use clap::Parser;


#[derive(Debug)]
#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(short, long)]
    config: Option<String>,
}

