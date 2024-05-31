#![no_std]
#![no_main]
// Make core::intrinsics work
#![feature(core_intrinsics)]
#![allow(internal_features)]

#[macro_use]
mod debug;

use core::arch::global_asm;
use core::intrinsics::abort;

// Define bootstrapping code executed at elf entry
// Usually rust would do that for us but it can't know what setup we want
global_asm! {
r#"
    .section .text

    .global _start
    _start:
        // Bootinfo pointer was put in rdi

        // Setup stack
        lea rsp, _stack_top
        mov rbp, rsp
        push rbp

        // 16-byte align stack pointer (required before calling a function)
        and rsp, 0xFFFFFFFFFFFFFFF0

        call main

        // Infinite loop in case we return
        loop: jmp loop
"#
}

// Define small, static stack
global_asm! {
r#"
    .section .bss
    .align 4096
        _stack_bottom:
    .space 16384
        _stack_top:
"#
}

#[no_mangle]
extern "C" fn main(_bootinfo: *const sel4_sys::seL4_BootInfo) {
    debug_println!("Hello World!");
    //debug_println!("{:?}", unsafe { *_bootinfo });
}

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    debug::debug_println!();
    debug::debug_println!("-- ROOT TASK PANICKED --");
    debug::debug_println!("{info}");
    debug::debug_println!("-- ROOT TASK PANICKED --");
    debug::debug_println!();

    abort()
}
