use std::path::PathBuf;

use crate::error::MockaError;

pub struct ServeConfig {
    pub directory: PathBuf,
    pub port: u16,
}

impl ServeConfig {
    pub fn new(directory: PathBuf, port: u16) -> Result<Self, MockaError> {
        if !directory.exists() {
            return Err(MockaError::Config(format!(
                "Cannot serve: directory {} does not exist",
                directory.display()
            )));
        }

        Ok(ServeConfig { directory, port })
    }
}

pub struct FetchConfig {
    pub url: String,
    pub output: PathBuf,
}

impl FetchConfig {
    pub fn new(url: String, output: PathBuf) -> Result<Self, MockaError> {
        if let Some(parent) = output.parent() {
            if !parent.exists() {
                return Err(MockaError::Config(format!(
                    "Output directory {} does not exist",
                    parent.display()
                )));
            }
        }

        // TODO: validate URL
        if url.is_empty() {
            return Err(MockaError::Config("URL cannot be empty".to_string()));
        }

        Ok(FetchConfig { url, output })
    }
}
