pub mod cli;
pub mod config;
pub mod error;
pub mod fetch;
pub mod server;
pub mod utils;

use cli::{Cli, Commands};

pub async fn run(cli: Cli) -> Result<(), error::MockaError> {
    match cli.command {
        Commands::Serve { directory, port } => {
            let config = config::ServeConfig::new(directory, port)?;
            let server = server::Server::new(config);
            server.run().await?
        }
        Commands::Fetch { url, output } => {
            let config = config::FetchConfig::new(url, output)?;
            fetch::fetch(config).await?
        }
    }
    Ok(())
}
