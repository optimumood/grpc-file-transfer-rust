use clap::Parser;
use std::net::IpAddr;
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long)]
    pub directory: PathBuf,
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub address: IpAddr,
    #[arg(short, long)]
    pub port: Option<u16>,
    #[arg(short, long, default_value = "info")]
    pub verbose: Level,
}
