#![no_std]
#![no_main]
// #![feature(asm)]

mod vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::print_something();
    // vga::print_ascii();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}