fn main() {
    let mut s = String::from("hello");

    let r1 = &s[0..5];
    let r2 = &s[0..5];
    println!("{} {}", r1, r2);


    // let r3 = &mut s[0..5]; // This line will cause a compile-time error
    s.push_str(", world");
    println!("{}", s);
}