#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use multiboot2_header::{HeaderTagFlag, HeaderTagISA, MbiTagType, RelocatableHeaderTag, RelocatableHeaderTagPreference, Multiboot2Header, Multiboot2BasicHeader};
#[macro_use]
pub mod vga;
pub mod interrupts;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn rust_main(_multiboot_information_address : *const Multiboot2BasicHeader) -> ! {
    init();
    // print!("Sample {}", 1.0/0.0);
    let mb2_hdr = unsafe { Multiboot2Header::from(_multiboot_information_address) };
    println!("{:#?}", mb2_hdr);
    // x86_64::instructions::interrupts::int3();
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