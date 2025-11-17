mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommands};
use docker::DockerClient;

#[tokio::main]
async fn main() {
   let args = Cli::parse();
   let docker_client = DockerClient::new();
   
   match args.command {
        Command::List { list_command } => match list_command {
            ListCommands::Containers { all } => {
                // if all {
                //     println!("Listing all containers...");
                // } else {
                //     println!("Listing running containers...");
                // }
                match docker_client.list_containers(all).await {
                    Ok(containers) => {
                        for container in containers {
                            println!("{}\t{}\t{}", container.id.unwrap_or_default(), 
                                     container.names.unwrap_or_default().join(","), 
                                     container.status.unwrap_or_default());
                        }
                    }
                    Err(e) => {
                        eprintln!( "Error listing containers: {}", e);
                    }
                }
            }
        }
    }
}
// cargo run 
// cargo run --quiet -- list help
// cargo run -- list containers -h
// cargo run -- list containers 
// cargo run -- list containers -a