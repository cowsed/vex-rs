#![no_std]
use vex_rs::{motor::Motor, Port};

#[no_mangle]
fn main() {
    let m = Motor::new(Port::Port1);
}
