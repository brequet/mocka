pub mod cli;
pub mod config;
pub mod error;
pub mod server;

use cli::Cli;

pub async fn run(cli: Cli) -> Result<(), error::MockaError> {
    let config = config::Config::from_cli(&cli)?;
    let server = server::Server::new(config);
    server.run().await
}
