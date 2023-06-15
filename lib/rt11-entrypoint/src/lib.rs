//! Linux ELF Entrypoint Generator
//!
//! This crate provides interfaces to generate entry-points for ELF executables
//! on linux. These entry-points can be used without any C runtime (e.g.,
//! `crt1`) or dynamic loader.
//!
//! ELF entry-points are highly platform dependent. Therefore, the
//! generated entry-points are minimal stubs that deal with any platform
//! specifics and then jump to a caller provided platform-independent
//! loader function. This loader must perform application setup according to
//! the ELF specification and then return the address of the entry-point of the
//! application to the stub. The stub will then jump to this entry-point, which
//! must not return.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod arch;

/// Entry-point Assembly Stub
///
/// This macro expands to the assembly of a Linux ELF entry-point stub. On all
/// platforms it creates a single symbol with the given name in the specified
/// section. The symbol is marked as global. It embeds platform-specific entry
/// code that calls into the provided loader function, passing the original
/// kernel provided stack pointer as only argument. Once this loader returns,
/// the stub jumps to the address returned by the loader, which itself must not
/// return.
///
/// Depending on the platform, auxiliary symbols or data may be emitted.
///
/// This expects the symbol name of the loader function as first argument
/// (e.g., `sym my_loader`).
///
/// In rust, the signatures of the loader and the application entry-point are:
///
/// `extern "C" fn loader(sp: *const core::ffi::c_void) -> rt11_ffi_efi::elfn::Size`
/// `extern "C" fn main() -> !`
#[macro_export]
macro_rules! assembly {
    ($section:expr, $symbol:expr) => {
        core::concat!(
            // Create an ELF section with the given name. Flag 'a' marks the
            // resulting segment as runtime allocated, 'x' ensures the pages
            // are executable at runtime. Lastly, 'progbits' sets the section
            // to contain program code and data.
            ".pushsection .", $section, ", \"ax\", ", $crate::arch::native::asm_prefix!("progbits"), ";\n",
            // Align the entry-point to the platform requirements.
            ".balign ", $crate::arch::native::entry_align!(), ";\n",
            // Mark the symbol as global so it can be found by the linker when
            // marked as ELF entry-point (or called by other objects).
            ".globl ", $symbol, ";\n",
            // Mark the symbol as function (for the ELF symbol table).
            ".type ", $symbol, ", STT_FUNC;\n",
            // Jump target for the symbol.
            $symbol, ":\n",
            // Custom jump-pad of the platform, or empty.
            $crate::arch::native::entry_custom_begin!($symbol),
            // Mark start of function via DWARF.
            ".cfi_startproc;\n",
            // Fill in the actual assembly instructions of the platform.
            $crate::arch::native::entry_code!($symbol),
            // Mark end of function via DWARF.
            ".cfi_endproc;\n",
            // Custom tail-pad of the platform, or empty.
            $crate::arch::native::entry_custom_end!($symbol),
            // Calculate the size of the symbol ('.' refers to the current
            // position).
            ".size ", $symbol, ", . - ", $symbol, ";\n",
            // Restore the section marker of the surrounding code.
            ".popsection;\n"
        )
    }
}

#[cfg(test)]
mod tests {
    use rt11_ffi_elf::elfn;

    // Dummy Entry-point
    //
    // Create a dummy entry-point called `rt11_entrypoint_test` with a loader
    // that simply returns the address of a dummy application entry-point.
    // These entry-points are not meant to be called, but only generated and
    // tested for existence and validity.
    core::arch::global_asm!(
        assembly!(".text.rt11_entrypoint_test", "rt11_entrypoint_test"),
        sym rt11_entrypoint_loader,
    );
    extern "C" fn rt11_entrypoint_loader(_sp: *const core::ffi::c_void) -> elfn::Size {
        rt11_entrypoint_main as usize as elfn::Size
    }
    extern "C" fn rt11_entrypoint_main() -> ! {
        core::panic!("Dummy entry-point");
    }

    // Test entry-point generation
    //
    // Create a reference to the generated entrypoint and verify its address
    // is non-zero. This will ensure that the linker has to correctly resolve
    // the address and verify the generated assembly.
    #[test]
    fn test_existence() {
        extern "C" {
            fn rt11_entrypoint_test() -> !;
        }

        assert!(rt11_entrypoint_test as usize != 0);
    }
}
