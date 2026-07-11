use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
#[command(
    version,
    about = "A high-performance Minecraft server written from scratch in Rust."
)]
pub struct Cli {
    #[arg(
        short = 's',
        long = "server-dir",
        value_name = "SERVER_DIR",
        default_value = "./",
        help = "Directory containing server files."
    )]
    pub server_dir: PathBuf,
}
