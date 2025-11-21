// Your function here
fn apply_to_each<F>(v:Vec<i32>, f: F) 
where F: Fn(i32) -> i32
{
    for x in v {
        println!("{}", f(x));
    }
}
fn main() {
    let numbers = vec![1, 2, 3, 4];
    // Call your function here
    apply_to_each(numbers, |x| x * 2);
    
}
