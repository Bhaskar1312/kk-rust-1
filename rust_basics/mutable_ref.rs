fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world");
    println!("{}", r1);
    s.push_str("!");
    println!("{}", s);
    // println!("{}", r1);
    
    // Uncommenting the next line will cause a compile-time error
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time
}