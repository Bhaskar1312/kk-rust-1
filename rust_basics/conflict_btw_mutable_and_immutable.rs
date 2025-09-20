fn main () {

    let mut s = String::from("hello");

    let r1 = &s; 
    let r2 = &s; 
    println!("{} {}", r1, r2);

    let r3 = &mut s; // This line will cause a compile-time error
    r3.push_str(", world");
    println!("{}", r3);

    // println!("{}", r1); // Uncommenting this line will also cause a compile-time error

    // one mutable reference or any number of immutable references
}