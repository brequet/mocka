use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve {
        #[arg(short, long)]
        directory: PathBuf,

        #[arg(short, long, default_value = "3250")]
        port: u16,
    },

    Fetch {
        #[arg(short, long)]
        url: String,

        #[arg(short, long)]
        output: PathBuf,
    },
}
