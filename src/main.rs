use clap::Parser;

mod commands;
use commands::cli::{Cli, Command};

const PORTER_DIRECTORY_NAME: &str = ".porter";

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

fn is_repository() -> Result<bool, std::io::Error> {
    std::fs::exists(PORTER_DIRECTORY_NAME)
}

fn create_repository(repository_name: &String) -> () {
    let is_repository_result = is_repository();
    let create_repository = match is_repository_result {
        Ok(false) => true,
        Ok(true) => {
            eprint!("already a porter repository");
            false
        }
        Err(error) => {
            eprint!("failed to check for repository: {}", error);
            false
        }
    };

    if !create_repository {
        return;
    }

    let repository = Repository {
        repository_name: repository_name.clone(),
    };

    let _ = std::fs::create_dir(PORTER_DIRECTORY_NAME);
    println!("repository {} created", repository.repository_name);
}
