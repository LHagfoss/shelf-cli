use clap::Parser;
use std::path::PathBuf;

use crate::config::{Cli, Commands};

mod commands;
mod config;

fn main() {
    let cli = Cli::parse();

    let cache_dir: PathBuf = dirs::cache_dir()
        .expect("Could not find system cache directory")
        .join("rust_shelf_buffer");

    match &cli.command {
        Commands::Copy { files } => {
            commands::copy::run(files, &cache_dir);
        }
        Commands::Paste { destination } => {
            commands::paste::run(destination.as_ref(), &cache_dir);
        }
    }
}
