#![no_std]
#![no_main]
use vex_rs::{
    competition, controller,
    controller::{Axis, Type::Primary},
    motor::Motor,
    screen, time, Color, PORT1, PORT2,
};

#[no_mangle]
fn main() {
    competition::set_opcontrol(op);
    competition::set_autonomous(auto);
}
fn auto() {}
fn op() {
    screen::clear_screen(Color::RED);
    let lmot: Motor = Motor::new(PORT1).reverse();
    let rmot: Motor = Motor::new(PORT2);
    loop {
        let lstick = controller::axis_value(Primary, Axis::LeftY);
        let rstick = controller::axis_value(Primary, Axis::RightY);

        lmot.set_voltage(12.0 * lstick);
        rmot.set_voltage(12.0 * rstick);

        time::delay_ms(20);
    }
}
