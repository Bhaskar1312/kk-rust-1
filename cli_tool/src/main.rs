use clap::Parser;

#[derive(Parser)]
#[command(name="greeter")]
#[command(about="A simple Cli tool to greet users", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = String::from("Hello"))]
    greeting: String,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    uppercase: bool,
}

fn main() {
    let args = Cli::parse();

    let mut message = format!("{}, {}", args.greeting, args.name);

    if args.uppercase {
        message = message.to_uppercase();
    }

    println!("{}", message);
}
// cargo run -- --name Bhaskar
// cargo run -- --name Bhaskar --greeting Hi --uppercase