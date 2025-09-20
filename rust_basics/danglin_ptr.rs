fn main() {
    let r;
    {
        let x = 42;
        r = &x; // `r` is a reference to `x`
        // `x` goes out of scope here
    }
    // `r` is now a dangling pointer because `x` has been dropped
    
    // println!("{}", r); // Uncommenting this line will cause a compile-time error
}