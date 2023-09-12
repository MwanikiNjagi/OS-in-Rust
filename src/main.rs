// this is a Freestanding Rust Binary
#![no_std]//not including the standard library
#![no_main]//telling the Rust compiler that we don't want to use the normal entry point chain

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> !{
    vga_buffer::print_something();

    loop{}
}

//attribute that defines the function that a compiler should invoke when a panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}


