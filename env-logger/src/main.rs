use env_logger;
use log::{ info, warn, error}

fn main() {
   env_logger::init();
   info!("This is an info message");
   warn!("This is a warning message");
   error!("This is an error message"); 
}
