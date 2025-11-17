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
                println!("Listing containers (all: {})", all);
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
            ListCommands::Images => {
                println!("Listing images...");
                // Future implementation for listing images goes here
                match docker_client.list_images().await {
                    Ok(images) => {
                        for image in images {
                            println!("{}\t{}", 
                                     image.id, 
                                     image.repo_tags.join(","));
                        }
                    }
                    Err(e) => {
                        eprintln!( "Error listing images: {}", e);
                    }
                }
            }
        },
        Command::Start { container_name } => {
            println!("Starting container: {}", container_name);
            docker_client.start_container(&container_name).await.unwrap_or_else(|e| {
                eprintln!("Error starting container {}: {}", container_name, e);
            });
        }
    }
}
// cargo run 
// cargo run --quiet -- help
// cargo run --quiet -- list help
// cargo run -- list containers -h
// cargo run -- list containers 
// cargo run -- list containers -a
// cargo run --quiet --  list images
// cargo run --quiet --  start angry_chebyshev