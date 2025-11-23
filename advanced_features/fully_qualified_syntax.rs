trait Driver {
    fn move_forward(&self);
}

trait Flyer {
    fn move_forward(&self);
}

struct Robot;

impl Driver for Robot {
    fn move_forward(&self) {
        println!("Robot is driving forward.");
    }
}
impl Flyer for Robot {
    fn move_forward(&self) {
        println!("Robot is flying forward.");
    }
}
impl Robot {
    fn move_forward(&self) {
        println!("Robot is moving forward in its own way.");
    }
}
fn main() {
    let robot = Robot;

    Driver::move_forward(&robot); // Calls the Driver implementation
    Flyer::move_forward(&robot);  // Calls the Flyer implementation
    robot.move_forward();         // Calls Robot's own method
}