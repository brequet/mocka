use clap::Parser;
use mocka::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    mocka::run(cli).await?;
    Ok(())
}
