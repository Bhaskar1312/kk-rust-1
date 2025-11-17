use env_logger::Builder;
use log::{ debug, info, warn, error};
use std::io::Write;

fn main() {
    // env_logger::init();
    Builder::new()
        .filter(None, log::LevelFilter::Info) // now even with RUST_LOG=debug, only info and above will be shown
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%s"),
                record.level(),
                record.args()
            )
        })
        .init();

    
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
}
// RUST_LOG=debug cargo run
// RUST_LOG=info cargo run
