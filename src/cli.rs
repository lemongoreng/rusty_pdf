use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Rusty PDF")]
#[command(version = "1.0")]
#[command(about = "A blazingly fast PDF tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Merge {
        #[arg(short, long)]
        output: PathBuf,
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },

    ExtractImages {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        dir: PathBuf,
    },

    Rotate {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long, default_value = "90")]
        angle: i64,
    },
}
