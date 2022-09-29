use core::arch::asm;

/// NOP instruction
#[inline(always)]
//pub fn nop() {
 //   unsafe { hint(0) };
//}
pub fn nop() {
    unsafe { asm!("nop"); };
}

/// WFI (Wait For Interrupt) makes the processor suspend
/// execution (Clock is stopped) until one of the following
/// events take place:
///    An IRQ interrupt
///    An FIQ interrupt
///    A Debug Entry request made to the processor.
#[inline(always)]
pub unsafe fn wfi() {
    asm!("wfi");
}

