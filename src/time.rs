use crate::api;

pub fn delay_ms(ms: u32) {
    unsafe {
        api::vexDelay(ms);
    }
}
