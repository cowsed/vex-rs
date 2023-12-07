extern crate alloc;
use newlib_alloc::Alloc;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;

use alloc::string::ToString;

use crate::api;
use crate::screen;
use crate::types::Color;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let s = _info.to_string();
    unsafe {
        loop {
            screen::clear_screen(Color::RED);
            screen::draw_text(0, 0, &s);
            api::vexDelay(250);

            screen::clear_screen(Color::BLACK);
            screen::draw_text(0, 0, &s);
            api::vexDelay(250);
        }
    }
}
