use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version("0.1.0"), about("porter"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Add(AddArgs),
}

#[derive(Args, Debug)]
struct AddArgs {
    #[arg(short('n'), long("name"))]
    name: String,
}

fn main() {
    let cli_args = Cli::parse();

    match &cli_args.command {
        Some(Command::Add(add_args)) => {
            println!("Creating new document: {:?}", add_args.name);
        }
        _ => (),
    };
}
