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
        long = "config-dir",
        value_name = "CONFIG_DIR",
        default_value = "config",
        help = "Directory containing configuration files."
    )]
    pub config_dir: PathBuf,
}
