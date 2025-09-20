use regex::Regex;

fn main() {
    let email: &str = "example@example.com";
    let email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    let re = Regex::new(email_pattern).unwrap();

    if re.is_match(email) {
        println!("Valid email address: {}", email);
    } else {
        println!("Invalid email address: {}", email);
    }

}
