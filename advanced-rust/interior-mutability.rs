use std::cell::RefCell;
// Your struct here
struct MyStruct {
    value: RefCell<i32>,
}
fn main() {
    let my_struct = MyStruct {
        value: RefCell::new(10), // Set value to 10 using struct
    };

    // Mutate the value and increase by 10
    *my_struct.value.borrow_mut() += 10;

    println!("Updated value: {}", my_struct.value.borrow()); // Use borrow here
}
