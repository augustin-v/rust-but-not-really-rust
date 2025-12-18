#![no_std]
#![no_main]
use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

extern "C" {
    fn printf(f: *const c_char) -> c_int;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> i32{
    unsafe {
        printf("Hello world".as_ptr() as *const i8);
    }
    0
}