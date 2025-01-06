use std::fs;

use reqwest::Client;

use crate::config::FetchConfig;
use crate::error::MockaError;
use crate::utils::normalize_path_display;

pub async fn fetch(config: FetchConfig) -> Result<(), MockaError> {
    let client = Client::new();

    let response = client
        .get(config.url.as_str())
        .send()
        .await
        .map_err(|err| MockaError::Http(err.to_string()))?;

    let status = response.status();
    if !status.is_success() {
        return Err(MockaError::Http(format!("Failed to fetch URL: {}", status)));
    }

    let content = response
        .bytes()
        .await
        .map_err(|err| MockaError::Http(err.to_string()))?;

    let file_path = config.get_file_path();
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            MockaError::Io(format!(
                "Failed to create directories ('{}'): {}",
                file_path.display(),
                e
            ))
        })?;
    }

    fs::write(&file_path, content)
        .map_err(|e| MockaError::Io(format!("Failed to write file: {}", e)))?;

    println!(
        "Successfully fetched URL and created: {}",
        normalize_path_display(&file_path)
    );

    Ok(())
}
