// Your struct here
struct BookExcerpt<'a>{
    content: &'a str,
}
// Do not modify
fn main() {
    let book = String::from("Rust Programming");
    let excerpt = BookExcerpt { content: &book[0..4] };
    println!("Excerpt: {}", excerpt.content);
}
