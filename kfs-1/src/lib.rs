#![no_std]
#![no_main]
// #![feature(asm)]

mod vga;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main() -> ! {
    use core::fmt::Write;
    // vga::clear_screen();
    // println!("Hello world ? {}", 1.337);
    vga::print_ft();
    // panic!("Oh no");
    // vga::new_line();
    writeln!(vga::WRITER.lock(), ", sample test {}", 1.0/2.0).unwrap();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}