#![no_std]
#![no_main]
// #![feature(asm)]

mod vga;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"victor salope";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}