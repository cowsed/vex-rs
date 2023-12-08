#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

use alloc::string::{String, ToString};

use crate::api;
use crate::screen;
use crate::types::Color;
use alloc::ffi::CString;

pub fn print(s: String){
    let s = CString::new(s).expect("failed to unwrap string");
    let s = s.as_c_str();
    unsafe{
        api::printf(s.as_ptr());
        api::vexControllerTextSet(crate::controller::Type::Primary.into(), 1,1, s.as_ptr());
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let chars_per_line = 35;
    let lines = _info.to_string()
        .chars()
        .collect::<Vec<char>>()
        .chunks(chars_per_line)
        .map(|c| c.iter().collect::<String>())
        .map(|str| CString::new(str).expect("error making panic string"))
        .collect::<Vec<CString>>();


    let mut col = Color::RED;

    unsafe {
        loop {
            screen::clear_screen(col);
            api::vexDisplayForegroundColor(0xFFFFFFF);
            for (num, line) in lines.iter().enumerate() {
                let s = line.as_c_str();
                api::vexDisplayStringAt(20, num as i32*20 + 20 , s.as_ptr());
            }
            api::vexDelay(250);
            api::vexDisplayRender(true, true);

            if col == Color::RED {
                col = Color::BLACK;
            } else {
                col = Color::RED;
            }
        }
    }
}
