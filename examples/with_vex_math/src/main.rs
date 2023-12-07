#![no_std]
#![no_main]
use vex_rs::{
    competition, controller,
    controller::{Axis, Type::Primary},
    motor::Motor,
    screen, time, Color, PORT1, PORT2,
};
extern crate alloc;
use alloc::string::ToString;

use vex_math::utils::{moving_average::{self, MovingAverage}, functional_command::CompletedCommandBuilder};
use vex_math::utils::functional_command::CommandBuilder;
use vex_math::abi;
use qunit::{angular_velocity::{AngularVelocity, AngularVelocityExt}, angle::AngleExt, length::LengthExt};
use qunit::angle::Angle;
#[derive(Debug)]
struct MyMotor{ mot: vex_rs::motor::Motor}

impl abi::MotorT for MyMotor{
    type MotorError = ();
    fn position(&self) -> Result<Angle, Self::MotorError> {
        return Ok(0.0.deg());
    }
    fn spin(&mut self, at: vex_math::utils::functional_command::runner::MotorControl) -> Result<(), Self::MotorError> {
        return Ok(());
    }
    fn velocity(&self) -> Result<AngularVelocity, Self::MotorError> {
        return Ok(0.0.degps())
    }
}
use qunit::length::Length;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Robot{}

impl vex_math::abi::CoreRobot for Robot{
    type Error = ();
    type Encoder = ();
    type MotorEnum = ();
    type Motor = MyMotor;
    type EncoderEnum = ();
    type Controller = ();
    type FlywheelEnum = ();
    type ImuEnum = ();
    type TankDriveEnum = ();
    type ImuT = ();
    fn radius(&self) -> qunit::length::Length {
        return 0.0.in_();
    }
}

#[no_mangle]
fn main() {
    op();
}



fn op() {
    let cmds:CompletedCommandBuilder<Robot> = CommandBuilder::new().example_command().build();


}
