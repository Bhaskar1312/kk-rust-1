// Your function here
fn longest_with_announcement<'a, T>(x:&'a str, y:&'a str, ann: T) -> &'a str 
where T: std::fmt::Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("Hello, world!");
    let string2 = "Rust";
    let result = longest_with_announcement(string1.as_str(), string2, "Comparing lengths");
    println!("Longest: {}", result);
}
