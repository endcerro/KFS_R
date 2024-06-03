#![no_std]
#![no_main]
// #![feature(asm)]

mod vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main() -> ! {
    vga::print_ft();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}