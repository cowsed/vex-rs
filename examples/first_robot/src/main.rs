#![no_std]
#![no_main]
use vex_rs::{motor::Motor, types::Port};

#[no_mangle]
fn main() {
    let m = Motor::new(Port::Port1);
    m.set_voltage(12.0);
}
