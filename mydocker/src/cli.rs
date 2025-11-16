use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="mydocker")]
#[command(about="A minimal Docker-like CLI tool", long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Command,
}


#[derive(Subcommand)]
pub enum Command {
    List {
        #[command(subcommand)]
        list_command: ListCommands,
    }
}

/// Enum for subcommands under the 'list' 
#[derive(Subcommand)]
pub enum ListCommands {
    /// List all containers
    Containers,
}