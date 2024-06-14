#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

// use multiboot2::{BootInformation, BootInformationHeader};
#[macro_use]
pub mod vga;
pub mod interrupts;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main(_multiboot_information_address: usize) -> ! {
    init();
    print!("Sample {}", 1.0/0.0);
    x86_64::instructions::interrupts::int3();
    loop {}
}

fn init() {
    vga::clear_screen();
    vga::print_ft();
    interrupts::init_idt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);
    loop {}
}