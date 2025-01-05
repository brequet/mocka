use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub directory: PathBuf,

    #[arg(short, long, default_value = "3250")]
    pub port: u16,
}
