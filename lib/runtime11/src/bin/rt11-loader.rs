//! Runtime11 Dynamic Loader
//!
//! XXX

#![no_std]
#![no_main]

use rt11_entrypoint;
use rt11_ffi_linux;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn exit(code: usize) -> usize {
    let sc = rt11_ffi_linux::native::syscall::Syscall { };

    unsafe {
        <_ as rt11_ffi_linux::common::Syscall>::syscall1(
            &sc,
            rt11_ffi_linux::native::nr::EXIT as usize,
            code,
        )
    }
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
);
