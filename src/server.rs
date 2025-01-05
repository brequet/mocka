use std::{path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde_json::json;
use tokio::fs;
use tower_http::trace::TraceLayer;

use crate::{config::ServeConfig, error::MockaError};
use tracing::{info, warn};

pub struct Server {
    config: Arc<ServeConfig>,
}

impl Server {
    pub fn new(config: ServeConfig) -> Self {
        Server {
            config: Arc::new(config),
        }
    }

    pub async fn run(&self) -> Result<(), MockaError> {
        let app = Router::new()
            .without_v07_checks()
            .route("/{*path}", get(Self::handle_request))
            .layer(TraceLayer::new_for_http())
            .with_state(self.config.clone());

        let addr = format!("127.0.0.1:{}", self.config.port);
        info!(
            "Serving folder {} on {}",
            self.config.directory.display(),
            addr
        );

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| MockaError::Server(format!("Failed to bind to address: {}", e)))?;

        axum::serve(listener, app)
            .await
            .map_err(|err| MockaError::Server(err.to_string()))
    }

    async fn handle_request(
        Path(path): Path<String>,
        State(state): State<Arc<ServeConfig>>,
    ) -> Response {
        info!("Handling request for path: {}", path);

        let mut file_path = state.directory.clone();
        let path = path.trim_start_matches('/');

        if path.is_empty() {
            return Self::handle_directory(&file_path).await;
        }

        let path_components: Vec<&str> = path.split('/').collect();
        for component in path_components {
            // TODO this doesn't seems to work, investigate
            if component == ".." || component.starts_with("../") {
                warn!("Attempted path traversal blocked for path: {}", path);
                return (StatusCode::FORBIDDEN, "Path traversal not allowed").into_response();
            }
            file_path.push(component);
        }

        if file_path.is_dir() {
            return Self::handle_directory(&file_path).await;
        }

        Self::serve_file(&file_path).await
    }

    async fn handle_directory(dir_path: &PathBuf) -> Response {
        match fs::read_dir(dir_path).await {
            Ok(mut entries) => {
                let mut directories = Vec::new();
                let mut files = Vec::new();

                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Ok(file_type) = entry.file_type().await {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if file_type.is_dir() {
                            directories.push(name);
                        } else {
                            files.push(name);
                        }
                    }
                }

                directories.sort();
                files.sort();

                info!(
                    "Serving directory listing for {} ({} directories, {} files)",
                    dir_path.display(),
                    directories.len(),
                    files.len()
                );

                let display_path = dir_path
                    .components()
                    .map(|comp| comp.as_os_str().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("/");
                let clean_path = display_path
                    .strip_prefix("./")
                    .unwrap_or(&display_path)
                    .to_string();

                let content = json!({
                    "path": clean_path,
                    "directories": directories,
                    "files": files
                });

                (
                    StatusCode::OK,
                    [("content-type", "application/json")],
                    serde_json::to_string_pretty(&content).unwrap(),
                )
                    .into_response()
            }
            Err(e) => {
                warn!("Failed to read directory '{}': {}", dir_path.display(), e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read directory: {}", e),
                )
                    .into_response()
            }
        }
    }

    async fn serve_file(file_path: &PathBuf) -> Response {
        let extensions = [".json", ".txt", ""];
        let mut file_content = None;

        for ext in extensions.iter() {
            let try_path = if ext.is_empty() {
                file_path.clone()
            } else {
                file_path.with_extension(ext.trim_start_matches('.'))
            };

            if let Ok(content) = fs::read(&try_path).await {
                let mime_type = mime_guess::from_path(&try_path)
                    .first_or_octet_stream()
                    .to_string();

                file_content = Some((content, mime_type));
                break;
            }
        }

        match file_content {
            Some((content, mime_type)) => {
                (StatusCode::OK, [("content-type", mime_type)], content).into_response()
            }
            None => {
                warn!("File not found: {}", file_path.display());
                (
                    StatusCode::NOT_FOUND,
                    format!("File not found: {}", file_path.display()),
                )
                    .into_response()
            }
        }
    }
}
