#![no_std]
#![no_main]
use vex_rs::{
    competition, controller,
    controller::{Axis, Type::Primary},
    motor::Motor,
    screen, time,
    util::print,
    Color, PORT1, PORT2, PORT3, PORT4,
};
extern crate alloc;
use alloc::format;
use alloc::string::ToString;

#[no_mangle] 
fn main() {
    op();
    competition::set_opcontrol(op);
    competition::set_autonomous(auto);
}
fn auto() {}
fn op() {
    screen::clear_screen(Color::RED);
    let lmot1: Motor = Motor::new(PORT1).reverse();
    let lmot2: Motor = Motor::new(PORT2).reverse();
    let rmot1: Motor = Motor::new(PORT3);
    let rmot2: Motor = Motor::new(PORT4);
    loop {
        let lstick = controller::axis_value(Primary, Axis::LeftY);
        let rstick = controller::axis_value(Primary, Axis::RightY);

        lmot1.set_voltage(12.0 * lstick);
        lmot2.set_voltage(12.0 * lstick);
        rmot1.set_voltage(12.0 * rstick);
        rmot2.set_voltage(12.0 * rstick);

        screen::clear_screen(Color::WHITE);

        let p: Option<vex_rs::Port> = Some(PORT1);
        screen::draw_text(20, 20, &format!("Port: {}", p.unwrap()), Color::BLACK);
        screen::render();
        time::delay_ms(20);
    }
}
