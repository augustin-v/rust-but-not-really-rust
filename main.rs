#![no_std]
#![no_main]
use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;

#[allow(non_camel_case_types)]
type intptr_t = isize;

extern "C" {
    fn printf(f: *const c_char, ...) -> c_int;
    fn sbrk(incr: intptr_t) -> *mut c_void;
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

pub mod array {
    use super::sbrk;
    #[repr(C)]
    pub struct Array<T> {
        pub data: *mut T,
        pub cap: usize,
        pub len: usize
    }

    pub unsafe  fn append<T>(array: *mut Array<T>, item: T) {
        if (*array).len >= (*array).cap {
            let size = size_of::<T>() as isize;

            let new_ptr = sbrk(32isize * size) as *mut T;
            let old_data = (*array).data as *mut T;

            // have to do this so that the old data doesn't just get lost
            // optimally we would also want to deallocate the memory for the old_ptr 
            // but it is difficult with sbrk only returning last break.
            // Also old values just leak, as they do not get zeroized it just 
            // copies the old data into a new buffer with more cap
            ptr::copy_nonoverlapping::<T>(old_data,  new_ptr, (*array).len);
            (*array).cap += 32;
            (*array).data = new_ptr;
        }
        *((*array).data.add((*array).len)) = item;
        (*array).len += 1;
    }

    pub unsafe fn new<T>() -> *mut Array<T> {
        let arr_ptr = sbrk((size_of::<Array<T>>() )as isize) as *mut Array<T>;
        (*arr_ptr).data = sbrk(( 32 *size_of::<T>()) as isize) as *mut T;
        (*arr_ptr).cap =32;
        (*arr_ptr).len = 0;
        
        arr_ptr
    }
}

#[no_mangle]
pub extern "C" fn main(
                            argc: c_int,  
                            argv: *const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const*const*const*const*const*mut*const u8,
                            // usually known as `argv: *mut*mut u8` since its supposed to be a pointer to a pointer of arguments. do not be scared
    ) -> i32{
    unsafe {
        printf("Hello world %d\n\0".as_ptr() as *const i8, argc);
        let bytes = 0isize;
        let start = sbrk(bytes);

        printf("break at %p\n\n\0".as_ptr() as *const i8, start);
        let arr = array::new::<u8>();
        for b in b"welcome to the abyss\n\n\0" {
            array::append(arr, *b);
        }

        printf("%s\n\0".as_ptr()as *const i8, (*arr).data );

        for i in 1..argc {
            let arg_ptr = *argv.add(i as usize);
            printf("arg %d: %s\n\0".as_ptr() as *const i8, i, arg_ptr as *const i8);
        }
        // free the allocated memory
        let total_bytes= size_of::<array::Array<u8>>() + (*arr).cap * size_of::<u8>();
        sbrk(-(total_bytes as isize));
    }
    0
}