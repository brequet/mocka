use std::path::PathBuf;
use url::Url;

use crate::{error::MockaError, utils::sanitize_filename};

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
    pub url: Url,
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

        let url =
            Url::parse(&url).map_err(|e| MockaError::Config(format!("Invalid URL: {}", e)))?;

        Ok(FetchConfig { url, output })
    }

    pub fn get_file_path(&self) -> PathBuf {
        let mut file_path = self.output.clone();

        let path = self.url.path().trim_start_matches('/');
        for segment in path.split('/') {
            if !segment.is_empty() {
                let safe_segment = sanitize_filename(segment);
                file_path.push(safe_segment);
            }
        }

        if let Some(query) = self.url.query() {
            let current_stem = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("index");
            let safe_query = sanitize_filename(query);
            let new_name = format!("{}-{}", current_stem, safe_query);
            file_path.set_file_name(new_name);
        }

        file_path
    }
}
