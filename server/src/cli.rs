use std::path::PathBuf;
use std::net::IpAddr;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long)]
    pub directory: PathBuf,
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub address: IpAddr,
    #[arg(short, long)]
    pub port: u16,
}
