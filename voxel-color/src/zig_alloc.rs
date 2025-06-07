extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn zig_alloc(size: usize, align: usize) -> *mut u8;
    fn zig_dealloc(ptr: *mut u8, size: usize, align: usize);
}

pub struct ZigAllocator;

unsafe impl GlobalAlloc for ZigAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        zig_alloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        zig_dealloc(ptr, layout.size(), layout.align())
    }
}


