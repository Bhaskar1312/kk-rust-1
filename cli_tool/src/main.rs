use clap::Parser;

#[derive(Parser)]
#[command(name="greeter")]
#[command(about="A simple Cli tool to greet users", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
// cargo run -- --name Bhaskar