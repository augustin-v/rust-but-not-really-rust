#![no_std]
#![no_main]
use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

type intptr_t = isize;

extern "C" {
    fn printf(f: *const c_char, ...) -> c_int;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

// TODO: implement custom array for **argv
#[no_mangle]
pub extern "C" fn main(argc: c_int) -> i32{
    unsafe {
        printf("Hello world %d".as_ptr() as *const i8, argc);
    }
    0
}