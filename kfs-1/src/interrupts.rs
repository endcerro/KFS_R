// #[repr(C)]
// pub struct InterruptDescriptorTable {
//     pub divide_by_zero: Entry<HandlerFunc>,
//     pub debug: Entry<HandlerFunc>,
//     pub non_maskable_interrupt: Entry<HandlerFunc>,
//     pub breakpoint: Entry<HandlerFunc>,
//     pub overflow: Entry<HandlerFunc>,
//     pub bound_range_exceeded: Entry<HandlerFunc>,
//     pub invalid_opcode: Entry<HandlerFunc>,
//     pub device_not_available: Entry<HandlerFunc>,
//     pub double_fault: Entry<HandlerFuncWithErrCode>,
//     pub invalid_tss: Entry<HandlerFuncWithErrCode>,
//     pub segment_not_present: Entry<HandlerFuncWithErrCode>,
//     pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
//     pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
//     pub page_fault: Entry<PageFaultHandlerFunc>,
//     pub x87_floating_point: Entry<HandlerFunc>,
//     pub alignment_check: Entry<HandlerFuncWithErrCode>,
//     pub machine_check: Entry<HandlerFunc>,
//     pub simd_floating_point: Entry<HandlerFunc>,
//     pub virtualization: Entry<HandlerFunc>,
//     pub security_exception: Entry<HandlerFuncWithErrCode>,
//     // some fields omitted
// }

use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT : InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(handler);
        idt.debug.set_handler_fn(handler);
        idt.non_maskable_interrupt.set_handler_fn(handler);
        idt.breakpoint.set_handler_fn(handler);
        idt.overflow.set_handler_fn(handler);
        idt.bound_range_exceeded.set_handler_fn(handler);
        idt.invalid_opcode.set_handler_fn(handler);
        idt.device_not_available.set_handler_fn(handler);
        idt.double_fault.set_handler_fn(handler_with_errcode_diverging);
        idt.invalid_tss.set_handler_fn(handler_with_errcode);
        idt.segment_not_present.set_handler_fn(handler_with_errcode);
        idt.stack_segment_fault.set_handler_fn(handler_with_errcode);
        idt.general_protection_fault.set_handler_fn(handler_with_errcode);
        idt.page_fault.set_handler_fn(handler_page_fault);
        idt.x87_floating_point.set_handler_fn(handler);
        idt.alignment_check.set_handler_fn(handler_with_errcode);
        idt.machine_check.set_handler_fn(handler_diverging);
        idt.simd_floating_point.set_handler_fn(handler);
        idt.virtualization.set_handler_fn(handler);
        idt.security_exception.set_handler_fn(handler_with_errcode);
        idt
    };
}

pub fn init_idt() {
    IDT.load();


}

use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;
extern "x86-interrupt" fn handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_with_errcode_diverging(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_diverging(
    stack_frame: InterruptStackFrame) -> !
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_with_errcode(
    stack_frame: InterruptStackFrame, _error_code: u64)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handler_page_fault(
    stack_frame: InterruptStackFrame, _error_code: PageFaultErrorCode)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


