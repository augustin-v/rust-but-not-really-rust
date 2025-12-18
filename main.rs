#![no_std]
#![no_main]
use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;

type intptr_t = isize;

extern "C" {
    fn printf(f: *const c_char, ...) -> c_int;
    fn sbrk(incr: isize) -> *mut c_void;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

mod Array {
    use super::sbrk;
    #[repr(C)]
    struct Array<T> {
        data: *mut T,
        cap: usize,
        len: usize
    }

    unsafe fn append<T>(array: *mut Array<T>, item: T) {
        if (*array).len >= (*array).cap {
            // keep somewhat conservative alignment
            sbrk(32isize);
            (*array).cap += 32;
        }
        *((*array).data.add((*array).len)) = item;
        (*array).len += 1;
    }
}

// TODO: implement custom array for **argv
#[no_mangle]
pub extern "C" fn main(argc: c_int) -> i32{
    unsafe {
        printf("Hello world %d\n\0".as_ptr() as *const i8, argc);
        let bytes = 0isize;
        let start = sbrk(bytes);

        printf("break at %p\n\0".as_ptr() as *const i8, start);
    }
    0
}