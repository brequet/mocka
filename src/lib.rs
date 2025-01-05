pub mod cli;
pub mod config;
pub mod error;
pub mod server;

use cli::{Cli, Commands};
use config::{FetchConfig, ServeConfig};
use tracing::{error, info};

pub async fn run(cli: Cli) -> Result<(), error::MockaError> {
    match cli.command {
        Commands::Serve { directory, port } => {
            let config = config::ServeConfig::new(directory, port)?;
            serve(config).await?
        }
        Commands::Fetch { url, output } => {
            let config = config::FetchConfig::new(url, output)?;
            fetch(config).await?
        }
    }
    Ok(())
}

async fn serve(config: ServeConfig) -> Result<(), error::MockaError> {
    let server = server::Server::new(config);
    server.run().await
}

async fn fetch(config: FetchConfig) -> Result<(), error::MockaError> {
    // TODO: implement
    info!("Fetching '{}' to {}", config.url, config.output.display());
    error!("Not implemented yet");
    Ok(())
}
