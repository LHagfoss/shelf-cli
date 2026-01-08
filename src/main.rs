use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(name = "shelf")]
#[command(version = "1.0")]
#[command(about = "A disk-based holding buffer for files and folders.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Copy {
        #[arg(required = true)]
        files: Vec<String>,
    },

    Paste {
        destination: Option<String>,
    },
}

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
