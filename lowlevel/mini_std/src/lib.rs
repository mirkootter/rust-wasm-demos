#![no_std]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]

extern crate alloc;

mod internal {
    extern "C" {
        #[link_name = "llvm.wasm.throw"]
        pub fn wasm_throw(tag: i32, ptr: *mut u8) -> !;
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn free(ptr: *mut u8);
        pub fn aligned_alloc(a: usize, b: usize) -> *mut u8;
        pub fn write(fd: i32, ptr: *const i8, size: usize) -> isize;
    }
}

mod libc_alloc {
    struct System;

    #[global_allocator]
    static ALLOC: System = System;

    unsafe impl alloc::alloc::GlobalAlloc for System {
        unsafe fn alloc(&self, layout: alloc::alloc::Layout) -> *mut u8 {
            crate::libc::aligned_alloc(layout.align(), layout.size()) as *mut u8
        }

        unsafe fn dealloc(&self, ptr: *mut u8, _: alloc::alloc::Layout) {
            crate::libc::free(ptr as *mut u8)
        }
    }
}

pub unsafe fn wasm_throw(ptr: *mut u8) -> ! {
    internal::wasm_throw(0, ptr);
}

pub fn print(msg: &str) {
    unsafe {
        libc::write(1 /* stdout */, msg.as_ptr() as *const i8, msg.len());
    }
}

#[cfg(not(test))]
#[lang = "start"]
fn lang_start<T>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _sigpipe: u8,
) -> isize {
    let _ = main();
    0
}
