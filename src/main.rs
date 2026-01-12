use clap::Parser;
mod cli;
mod commands;

use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    let result = match &args.command {
        Commands::Merge { output, files } => commands::merge::run(files, output),
        Commands::ExtractImages { input, dir } => commands::extract::run(input, dir),
        Commands::Rotate {
            input,
            output,
            angle,
        } => commands::rotate::run(input, output, *angle),
    };

    if let Err(e) = result {
        eprintln!("Application Error: {}", e);
        std::process::exit(1);
    } else {
        println!("Operation completed successfully.");
    }
}
