use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="greeter")]
#[command(about="A simple Cli tool to greet users", long_about = None)]
struct Cli {
    // #[arg(short, long)]
    // name: String,

    // #[arg(short, long, default_value_t = String::from("Hello"))]
    // greeting: String,

    // #[arg(short, long, action = clap::ArgAction::SetTrue)]
    // uppercase: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello {
        #[arg(short, long)]
        name: String, 
    },
    Goodbye {
        #[arg(short, long)]
        name: String,
    }
}

fn main() {
    let args = Cli::parse();

    // let mut message = format!("{}, {}", args.greeting, args.name);

    // if args.uppercase {
    //     message = message.to_uppercase();
    // }

    // println!("{}", message);

    match args.command {
        Commands::Hello { name } => {
            println!("Hello {}!", name);
        }
        Commands::Goodbye { name } => {
            println!("Goodbye {}!", name);
        }
    }
    // cargo run -- goodbye --name Bhaskar
    // cargo run -- hello --name Bhaskar
}
// cargo run -- --name Bhaskar
// cargo run -- --name Bhaskar --greeting Hi --uppercase