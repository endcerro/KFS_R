#![no_std]
#![no_main]
// #![feature(asm)]

mod vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main() -> ! {
    use core::fmt::Write;
    vga::clear_screen();
    vga::print_ft();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}