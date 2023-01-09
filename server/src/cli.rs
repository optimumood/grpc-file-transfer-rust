use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(short, long)]
    directory: PathBuf,
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    hostname: String,
    #[arg(short, long)]
    port: Option<u16>,
}
