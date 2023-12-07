pub struct Alloc;
use crate::api;

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        api::malloc(layout.size()) as *mut _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        api::free(ptr as *mut _);
    }
}

#[global_allocator]
static ALLOCATOR: Alloc = Alloc;
use core::alloc::{GlobalAlloc, Layout};
