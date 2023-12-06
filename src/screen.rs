use crate::api;
use crate::types::{Rect, Color};

extern crate alloc;
use alloc::ffi::CString;
use alloc::string::String;


pub fn clear_screen(c: Color) {
    let full_screen = Rect {
        x1: 0,
        y1: 0,
        x2: 480,
        y2: 240,
    };
    draw_rectangle(full_screen, c, Some(c))
}

pub fn draw_text(x: i32, y: i32, s: &String) {
    let s = CString::new(s.as_str()).expect("Couldn't form CString from string passed to draw_text");
    let s = s.as_c_str();
    unsafe { api::vexDisplayStringAt(x, y, s.as_ptr()) }
}

pub fn draw_rectangle(r: Rect, stroke: Color, fill: Option<Color>) {
    if fill.is_some() {
        unsafe {
            api::vexDisplayForegroundColor(stroke.into());
            api::vexDisplayBackgroundColor(fill.unwrap().into());
            api::vexDisplayRectFill(r.x1, r.y1, r.x2, r.y2);
        }
    } else {
        unsafe {
            api::vexDisplayForegroundColor(stroke.into());
            api::vexDisplayRectClear(r.x1, r.y1, r.x2, r.y2);
        }
    }
}
