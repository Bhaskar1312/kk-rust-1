fn never_stop1() -> ! {
    panic!("This function never returns!");
}
fn never_stop2() -> ! {
    loop {
        println!("This function loops forever!");
    }
}

