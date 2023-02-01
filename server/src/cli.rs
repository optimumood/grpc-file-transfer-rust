use clap::{builder::ArgPredicate, Parser};
use std::{net::IpAddr, path::PathBuf};
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
    #[arg(
        short,
        long,
        requires = "key",
        required = true,
        default_value_if("insecure", ArgPredicate::IsPresent, None)
    )]
    pub cert: Option<PathBuf>,
    #[arg(
        short,
        long,
        requires = "cert",
        required = true,
        default_value_if("insecure", ArgPredicate::IsPresent, None)
    )]
    pub key: Option<PathBuf>,
    #[arg(short, long, conflicts_with_all = ["key", "cert"])]
    pub insecure: bool,
}
