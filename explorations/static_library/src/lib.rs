#![no_std] // no std
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(alloc_error_handler)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate alloc; // yes alloc
use core::{alloc::Layout};
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

#[alloc_error_handler]
fn handle(layout: Layout) -> ! {
    panic!("memory allocation failed: {:#?}", layout);
}

use alloc::{
    ffi::CString,
    string::{String, ToString},
};

pub fn safe_print(s: String) {
    unsafe {
        let str = CString::new(s).expect("CString::new failed");
        vex_printf(str.as_c_str().as_ptr());
        vexDisplayBackgroundColor(0xFFFFFFFF);
        vexDisplayForegroundColor(0x00FF0000);
        vexDisplayBigStringAt(20, 100, str.as_ptr());
    }
}

unsafe extern "C" fn opcontrol() {
    let left_front_mot = vexDeviceGetByIndex(vex_PORT1 as u32);
    let left_rear_mot = vexDeviceGetByIndex(vex_PORT2 as u32);
    let right_front_mot = vexDeviceGetByIndex(vex_PORT3 as u32);
    let right_back_mot = vexDeviceGetByIndex(vex_PORT4 as u32);

    loop {
        let lstick = (vexControllerGet(
            _V5_ControllerId_kControllerMaster,
            _V5_ControllerIndex_AnaLeftY,
        ) as f32)
            / 127.0;
        let rstick = (vexControllerGet(
            _V5_ControllerId_kControllerMaster,
            _V5_ControllerIndex_AnaRightY,
        ) as f32)
            / 127.0;

        let max_motor = 100000.0;
        let lmot_volts = (max_motor * lstick) as i32;
        let rmot_volts = (max_motor * rstick) as i32;

        // vexDisplayBackgroundColor(0x0000000);
        // vexDisplayForegroundColor(0x0000000);
        // vexDisplayRectFill(0, 0, 300, 200);
        // safe_print(lstick.to_string() + " lstick");
        //
        vexDeviceMotorVoltageSet(left_rear_mot, -lmot_volts);
        vexDeviceMotorVoltageSet(left_front_mot, -lmot_volts);
        vexDeviceMotorVoltageSet(right_front_mot, rmot_volts);
        vexDeviceMotorVoltageSet(right_back_mot, rmot_volts);
        // vexDebug(s.as_c_str().as_ptr());

        vexDelay(20);
    }
}

#[no_mangle]
pub extern "C" fn main() {
    safe_print("Hello from rust\n".into());

    unsafe {
        let mut comp: vex_competition = vex_competition::new();
        comp.drivercontrol(Some(opcontrol));
    }
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let s = _info.to_string();
    let s = CString::new(s).unwrap();
    let s = s.as_c_str();

    unsafe {
        loop {
            vexDisplayBackgroundColor(ClrRed);
            vexDisplayForegroundColor(ClrRed);
            vexDisplayRectFill(0, 0, 480, 232);
            vexDisplayForegroundColor(ClrWhite);
            vexDisplayTextSize(5, 5);
            vexDisplayString(1, s.as_ptr());
            vexDisplayRender(true, true);
            vexDelay(250);

            vexDisplayBackgroundColor(ClrBlack);
            vexDisplayForegroundColor(ClrBlack);
            vexDisplayRectFill(0, 0, 480, 240);
            vexDisplayForegroundColor(ClrWhite);
            vexDisplayTextSize(5, 5);
            vexDisplayString(1, s.as_ptr());
            vexDisplayRender(true, true);
            vexDelay(250);
        }
    }
}
