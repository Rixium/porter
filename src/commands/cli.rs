use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version("0.1.0"), about("porter"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Create(CreateArgs),
    Add(AddArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    #[arg(short('n'), long("name"))]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short('n'), long("name"))]
    pub name: String,
}
