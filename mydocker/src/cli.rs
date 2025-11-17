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
    /// List resources
    List {
        /// Subcommand for listing resources
        #[command(subcommand)]
        list_command: ListCommands,
    },
    /// Start a container
    Start {
        /// Name of the container
        container_name: String,
    }
}

/// Enum for subcommands under the 'list' 
#[derive(Subcommand)]
pub enum ListCommands {
    /// List all containers
    Containers {
        /// Include stopped containers, short flag -a and long flag --all
        #[arg(short, long)]
        all: bool,
    },
    /// List images
    Images,
}