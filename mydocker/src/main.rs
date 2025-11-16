mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};

fn main() {
   let args = Cli::parse();
   
   match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers => {
                println!("Listing all containers...");

            }
        }
    }
}
// cargo run 
