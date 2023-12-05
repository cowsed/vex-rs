#![no_std] // no std
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(alloc_error_handler)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate alloc; // yes alloc
use core::alloc::Layout;
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
    // motor 4 left fwd
    // motor 2 right reversred
    // uint32_t              vexDevicesGetNumberByType( V5_DeviceType type );
    // V5_DeviceT            vexDevicesGet( void );
    // V5_DeviceT            vexDeviceGetByIndex( uint32_t index );
    let lmot = vexDeviceGetByIndex(vex_PORT2 as u32);

    let rmot = vexDeviceGetByIndex(vex_PORT4 as u32);

    safe_print("Hello from opcontrol\n".into());
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

        let max_motor = 10000.0;
        let lmot_volts = (max_motor * lstick) as i32;
        let rmot_volts = (max_motor * rstick) as i32;
        vexDisplayBackgroundColor(0x0000000);
        vexDisplayForegroundColor(0x0000000);
        vexDisplayRectFill(0, 0, 300, 200);
        safe_print(lstick.to_string() + " lstick");

        vexDeviceMotorVoltageSet(lmot, -lmot_volts);
        vexDeviceMotorVoltageSet(rmot, rmot_volts);
        vexDelay(20);
    }
}

#[no_mangle]
pub extern "C" fn main() {
    safe_print("Hello from rust2\n".into());

    unsafe {
        let mut comp: vex_competition = vex_competition::new();
        comp.drivercontrol(Some(opcontrol));
    }
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
