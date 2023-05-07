#![no_std]
#![feature(start)]
#![feature(core_intrinsics)]
#![feature(panic_info_message)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;

struct LogOnDrop;

impl Drop for LogOnDrop {
    fn drop(&mut self) {
        mini_std::print("Dropped\n");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    use crate::alloc::string::ToString;
    
    let msg = info.message().and_then(|msg| msg.as_str()).unwrap_or("(no message)");
    let exception = Box::new(msg.to_string());
    unsafe {
        let exception_raw = Box::into_raw(exception);
        mini_std::wasm_throw(exception_raw as *mut u8);
    }
}

#[allow(unreachable_code)]
fn main() {
    let data = 0x1234usize as *mut u8; // Something to recognize

    unsafe {
        core::intrinsics::r#try(|data: *mut u8| {
            let _log_on_drop = LogOnDrop;

            mini_std::print(&alloc::format!("`r#try` called with ptr {:?}\n", data));
            panic!("Oops");

            mini_std::print("This line should not be visible! :(\n");
        }, data, |data, exception| {
            let exception = *Box::from_raw(exception as *mut String);
            mini_std::print("Caught something!\n");
            mini_std::print(&alloc::format!("  data     : {:?}\n", data));
            mini_std::print(&alloc::format!("  exception: {:?}\n", exception));
        });
    }

    mini_std::print("This program terminates correctly.\n");
}
