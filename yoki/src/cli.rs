use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(
    version,
    about = "A high-performance Minecraft server written from scratch in Rust."
)]
pub struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "CONFIG_PATH",
        default_value = "server.toml",
        help = "Configuration file path"
    )]
    pub config_path: PathBuf,
}
