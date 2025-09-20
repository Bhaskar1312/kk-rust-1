macro_rules! print_custom_message {
    ($msg: expr)=> {
        println!("{}", $msg)
    }
}

fn main() {
  print_custom_message!("Hello from macro!");
  print_custom_message!("This is another message.");
}
