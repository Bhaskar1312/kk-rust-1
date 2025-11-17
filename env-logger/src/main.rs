use env_logger;
use log::{ debug, info, warn, error};

fn main() {
    env_logger::init();
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
}
// RUST_LOG=debug cargo run
// RUST_LOG=info cargo run
