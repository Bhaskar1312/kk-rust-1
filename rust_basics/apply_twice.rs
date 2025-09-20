fn main() {
    println!("{}", apply_twice(double, 4));
    println!("{}", apply_twice(increment, 5));
}
fn apply_twice(f: fn(i32)->i32, a: i32)-> i32 {
    return f(f(a));
}

fn double(a: i32) -> i32 {
    a * 2
}

fn increment(a: i32) -> i32 {
    a + 1
}