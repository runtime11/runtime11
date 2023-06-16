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

/// ARM EHABI Compact Personality Routines
///
/// The ARM EHABI defines a compact format for exception handler tables, which
/// embed the index of the pre-defined personality routine to use. Those
/// routines are called `__aeabi_unwind_cpp_pr?()` (with only `0`, `1`, and `2`
/// currently being defined). It is up to the platform to implement these
/// pre-defined routines. They have to follow the ARM-EHABI specification.
///
/// The `core` crate is always compiled with `panic=unwind`, but allows linking
/// into `panic=abort` applications. This avoids shipping `core` twice with
/// every compiler release. In most cases, final linking strips all unwind
/// tables and references. However, for some reason this does not reliably work
/// on ARM and the exception-tables remain. These contain dummy-relocations
/// that reference the pre-defined personality routines. This either is a
/// limitation of the platform, or a limitation of rustc. Regardless, we have
/// to provide those to satisfy the linker, even though we never end up raising
/// exceptions in our panic handler.
///
/// We simply return `_URW_FAILURE` (`9`), an ARM-specific unwinding reason
/// denoting failure to run the personality routine. But since they should
/// really never be called, this should not matter much.
///
/// XXX: We shouldn't be required to copy these everywhere. For now, we keep
///      them here, but we should either fix rustc or find a better home.
#[cfg(target_arch = "arm")]
#[no_mangle]
extern "C" fn __aeabi_unwind_cpp_pr0(
    _state: u32,
    _cb: *const core::ffi::c_void,
    _ctx: *const core::ffi::c_void,
) -> i32 {
    return 9;
}

/// ARM EHABI Compact Personality Routines
///
/// See `__aeabi_unwind_cpp_pr0()` for details.
#[cfg(target_arch = "arm")]
#[no_mangle]
extern "C" fn __aeabi_unwind_cpp_pr1(
    _state: u32,
    _cb: *const core::ffi::c_void,
    _ctx: *const core::ffi::c_void,
) -> i32 {
    return 9;
}

/// ARM EHABI Compact Personality Routines
///
/// See `__aeabi_unwind_cpp_pr0()` for details.
#[cfg(target_arch = "arm")]
#[no_mangle]
extern "C" fn __aeabi_unwind_cpp_pr2(
    _state: u32,
    _cb: *const core::ffi::c_void,
    _ctx: *const core::ffi::c_void,
) -> i32 {
    return 9;
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
