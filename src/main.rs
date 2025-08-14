use clap::Parser;

mod commands;
use commands::cli::{Cli, Command};

fn main() {
    let cli_args: Cli = Cli::parse();

    match &cli_args.command {
        Some(Command::Create(create_args)) => {
            create_repository(&create_args.name);
        }
        Some(Command::Add(add_args)) => {
            println!("Creating new document: {:?}", add_args.name);
        }
        _ => (),
    };
}

struct Repository {
    repository_name: String,
}

fn create_repository(repository_name: &String) {
    let repository = Repository {
        repository_name: repository_name.clone(),
    };

    println!("repository {} created", repository.repository_name);
}
