use clap::Parser;
use morum::{Config, Error};
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: String,
}

async fn run() -> Result<(), Error> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let config: Config = serde_yaml::from_str(&fs::read_to_string(args.config)?)?;

    let matrix = morum::matrix::start(config.clone()).await?;

    morum::web::start(config, matrix).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await?;
    Ok(())
}
