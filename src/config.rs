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
    Copy {
        #[arg(required = true)]
        files: Vec<String>,
    },

    Paste {
        destination: Option<String>,
    },
}
