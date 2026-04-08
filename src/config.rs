use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shelf")]
#[command(version)]
#[command(about = "A \"git stash\" like command, for files and folders.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Copy files/folders to the shelf.
    Copy {
        #[arg(required = true)]
        files: Vec<String>,
    },

    /// Add files/folders to the shelf.
    Add {
        #[arg(required = true)]
        files: Vec<String>,
    },

    /// Paste files/folders from the shelf to a destination, does not clean up the shelf.
    Paste { destination: Option<String> },

    /// Pop files/folders from the shelf to a destination, then cleanes up the shelf.
    Pop { destination: Option<String> },

    /// Drop a file/folder from the shelf.
    Drop {
        #[arg(required = true)]
        item: String,
    },

    /// Peak at the contents of the shelf.
    Peak,

    /// Clear the shelf.
    Clear,

    /// About the Shelf cli tool
    About,
}
