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
            commands::copy(files, &cache_dir);
        }
        Commands::Add { files } => {
            commands::add(files, &cache_dir);
        }
        Commands::Paste { destination } => {
            commands::paste(destination.as_ref(), &cache_dir);
        }
        Commands::Pop { destination } => {
            commands::pop(destination.as_ref(), &cache_dir);
        }
        Commands::Drop { item } => {
            commands::drop(item, &cache_dir);
        }
        Commands::Peak => {
            commands::peak(&cache_dir);
        }
        Commands::Clear => {
            commands::clear(&cache_dir);
        }
    }
}
