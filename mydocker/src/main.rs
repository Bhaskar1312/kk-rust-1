mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};

fn main() {
   let args = Cli::parse();
   
   match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                if all {
                    println!("Listing all containers...");
                } else {
                    println!("Listing running containers...");
                }
            }
        }
    }
}
// cargo run 
// cargo run --quiet -- list help
// cargo run -- list containers
// cargo run -- list containers -a