use clap::Parser;

mod commands;
use commands::cli::{Cli, Command};

fn main() {
    let cli_args: Cli = Cli::parse();

    match &cli_args.command {
        Some(Command::Add(add_args)) => {
            println!("Creating new document: {:?}", add_args.name);
        }
        _ => (),
    };
}
