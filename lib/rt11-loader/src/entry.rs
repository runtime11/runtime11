//! Dynamic Loader Entry-point
//!
//! This module provides the required stub that implements the entry-point of
//! the dynamic loader. This stub is meant to hide architecture specifics and
//! jump into the generic loader.

#[doc(hidden)]
#[macro_export]
macro_rules! entry_assembly {
    ($section:expr, $symbol:expr) => {
        core::concat!(
            ".pushsection .", $section, ", \"ax\", ", $crate::arch::native::asm_prefix!("progbits"), ";\n",
            ".balign ", $crate::arch::native::entry_align!(), ";\n",
            ".globl ", $symbol, ";\n",
            ".type ", $symbol, ", STT_FUNC;\n",
            $symbol, ":\n",
            $crate::arch::native::entry_custom_begin!($symbol),
            ".cfi_startproc;\n",
            $crate::arch::native::entry_code!($symbol),
            ".cfi_endproc;\n",
            $crate::arch::native::entry_custom_end!($symbol),
            ".size ", $symbol, ", .-", $symbol, ";\n",
            ".popsection;\n"
        )
    }
}

/// Dynamic Loader Entry-point
///
/// This macro expands to the assembly of the entry-point used for the dynamic
/// loader. On all architectures, it creates a single symbol with the given
/// name in the specified section. The symbol is marked as global. It embeds
/// architecture-specific entry code that calls into the generic loader,
/// passing the stack pointer as only argument. Once this returns, it jumps to
/// the address returned by the generic loader, which itself must not return.
///
/// Depending on the architecture, auxiliary symbols or data may be emitted.
///
/// The assembly uses AT&T syntax on all architectures. Hence,
/// `option(att_syntax)` must be used with rust inline assembly. Furthermore,
/// this expects the symbol name of the generic loader entry-point as first
/// argument (e.g., `sym rt11_loader_main`).
pub use entry_assembly as assembly;
