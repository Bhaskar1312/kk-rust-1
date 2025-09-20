fn main() {
    let mut outer_count = 0;
    'outer: loop {
        let mut inner_count = 0;
        loop {
            println!("Inner count: {}", inner_count);
            inner_count += 1;
            if inner_count == 2 {
                break;
            }
            if outer_count == 2 {
                break 'outer;
            }
        }
        outer_count += 1;
    }
    println!("Exited the outer loop");
}