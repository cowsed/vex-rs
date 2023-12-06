#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate alloc;
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

// #[alloc_error_handler]
// fn handle(layout: Layout) -> ! {
    // panic!("memory allocation failed: {:#?}", layout);
// }

use alloc::{
    ffi::CString,
    string::ToString,
};




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
