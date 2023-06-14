//! Runtime11 Dynamic Loader
//!
//! XXX

#![no_std]
#![no_main]

use rt11_entrypoint;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn exit(code: usize) -> usize {
    let r: usize;
    let nr: usize = 60;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") code,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }

    r
}

pub extern "C" fn main() -> ! {
    exit(71);
    core::panic!();
}

pub extern "C" fn loader_main(_sp: *const core::ffi::c_void) -> usize {
    main as usize
}

core::arch::global_asm!(
    rt11_entrypoint::assembly!(".text", "_start"),
    sym loader_main,
    options(att_syntax),
);
