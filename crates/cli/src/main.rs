use crate::{commands::Commands, error::Result};
use clap::Parser;

mod commands;
mod error;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct UciSettings {
    pub settings: serde_json::Value,
    pub secrets: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Generate(generate) => {
            let file_content = tokio::fs::read_to_string(generate.file).await?;
            let json = serde_json::from_str::<UciSettings>(&file_content)?;
            println!("{:?}", json);
        }
    }

    Ok(())
}
