use std::path::PathBuf;

use crate::{cli::Cli, error::MockaError};

pub struct Config {
    pub directory: PathBuf,
    pub port: u16,
}

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self, MockaError> {
        if !cli.directory.exists() {
            return Err(MockaError::Config(format!(
                "Directory {} does not exist",
                cli.directory.display()
            )));
        }

        Ok(Config {
            directory: cli.directory.clone(),
            port: cli.port,
        })
    }
}
