//! Architecture Specifics
//!
//! This module encapsulates all architecture specifics of the entry-point.
//! Every architecture specific bit of the entry-point is extracted into a
//! macro, function, etc., that is then implemented for each supported
//! architecture.
//!
//! Every supported architecture is represented by a sub-module, implementing
//! the same API. The architecture native to the compilation target is also
//! exposed via the `native` alias.
//!
//! Note that all architecture module are compiled unconditionally. Only the
//! bits that cannot be compiled on foreign platforms (e.g., inline assembly)
//! is guarded by conditional compilation. This improves compilation-coverage
//! and allows partial testing of foreign platforms. Furthermore, it allows
//! using this module to interact with foreign platforms even at runtime (e.g.,
//! in debug utilities).

// Target Architecture
//
// We need architecture-dependent assembly to implement entry-points. To avoid
// spurious linker errors in dependent crates, we check for supported
// architectures here and error out right away.
#[cfg(not(any(
    doc,
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "x86",
    target_arch = "x86_64",
)))]
compile_error!("Target architecture not supported.");

/// Architecture Support
///
/// This module documents the macros and interfaces required for each
/// architecture support module. It is a dummy module not available at
/// runtime.
///
/// Each supported architecture has a module under `arch` named after
/// the architecture. Different versions of an architecture should use
/// separate modules, if possible (e.g., `riscv32` and `riscv64` would
/// be separate). Names are picked similar to the architecture backends
/// of the linux kernel.
///
/// Macros must be exported, hidden, and prefixed with `arch_<id>_`.
/// Each macro is then aliased via `pub use arch_<id>_<macro> as <macro>`
/// to export it with a correct path.
///
/// Inline assembly must use AT&T syntax for all architectures.
#[cfg(doc)]
pub mod doc {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_doc_asm_prefix {
        ($id:literal) => { $id }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_doc_entry_align {
        () => { 0 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_doc_entry_code {
        ($_:expr) => { "" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_doc_entry_custom_begin {
        ($_:expr) => { "" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_doc_entry_custom_end {
        ($_:expr) => { "" }
    }

    /// Expand Identifier with Prefix
    ///
    /// Take a GNU-as identifier and prefix it with the
    /// architecture-dependent character. In most cases this uses `%`
    /// to prefix identifiers, but on some architectures this is used
    /// to start comments and thus `@` is used.
    ///
    /// Consult the GNU-as documentation for details on which character
    /// is used for a specific platform.
    pub use arch_doc_asm_prefix as asm_prefix;

    /// Alignment Requirements for Function Entry-points
    ///
    /// This macro expands to the alignment-requirements of function
    /// entry-points in bytes. Most commonly, functions are aligned
    /// to 16-byte boundaries, so this would return 16.
    pub use arch_doc_entry_align as entry_align;

    /// Entry-point Code
    ///
    /// This macro expands to the inline-assembly of the code used to
    /// implement the entry-point of the binary. It can use `{0}` to refer
    /// to the symbol name of the generic dynamic loader.
    pub use arch_doc_entry_code as entry_code;

    /// Custom Entry-point Header
    ///
    /// This macro expands to the custom header of an entry-point. If
    /// the architecture has no special requirements, this should expand
    /// to an empty string.
    ///
    /// Note that this is expanded _after_ the symbol label is declared,
    /// but before any code.
    pub use arch_doc_entry_custom_begin as entry_custom_begin;

    /// Custom Entry-point Footer
    ///
    /// This macro expands to the custom footer of an entry-point. If
    /// the architecture has no special requirements, this should expand
    /// to an empty string.
    ///
    /// Note that this is expanded after the code of the entry-point but
    /// before the generic footer of a symbol.
    pub use arch_doc_entry_custom_end as entry_custom_end;
}

/// ARM 32-bit Architecture Support
///
/// This module implements the required macros and interfaces for the
/// ARM architecture in 32-bit mode.
pub mod arm {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm_asm_prefix {
        // The '%' is used to denote GNU-AS identifiers for ARM.
        ($id:literal) => { concat!("%", $id) }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm_entry_align {
        // Use unaligned function entry-points.
        () => { 0 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm_entry_code {
        ($_:expr) => {
            core::concat!(
                // Mark the IP as undefined from here on, effectively
                // marking this as the last frame for unwinding.
                ".cfi_undefined r14;\n",

                // Call the loader with the stack-pointer as only
                // argument (in %r0). The loader will return the
                // application entry-point in %r0.
                "mov r0, sp;\n",
                "bl {0};\n",

                // Jump to the application entry-point with the same
                // stack as the kernel provided to us.
                "bx r0;\n",
            )
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm_entry_custom_begin {
        // Denote start of function with ARM-specific debug hint.
        ($_:expr) => { ".fnstart;\n" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm_entry_custom_end {
        // Denote end of function with ARM-specific debug hint.
        ($_:expr) => { ".fnend;\n" }
    }

    pub use arch_arm_asm_prefix as asm_prefix;
    pub use arch_arm_entry_align as entry_align;
    pub use arch_arm_entry_code as entry_code;
    pub use arch_arm_entry_custom_begin as entry_custom_begin;
    pub use arch_arm_entry_custom_end as entry_custom_end;
}

/// ARM 64-bit Architecture Support
///
/// This module implements the required macros and interfaces for the
/// ARM architecture in 64-bit mode.
///
/// ARMv8.5-A support is required at compile-time, but not runtime.
pub mod arm64 {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm64_asm_prefix {
        // The '%' is used to denote GNU-AS identifiers for ARM64.
        ($id:literal) => { concat!("%", $id) }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm64_entry_align {
        // Use 16-byte aligned function entry-points.
        () => { 16 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm64_entry_code {
        ($_:expr) => {
            core::concat!(
                // Mark the IP as undefined from here on, effectively
                // marking this as the last frame for unwinding.
                ".cfi_undefined x30;\n",

                // Call the loader with the stack-pointer as only
                // argument (in %x0). The loader will return the
                // application entry-point in %x0.
                "mov x0, sp;\n",
                "bl {0};\n",

                // Jump to the application entry-point with the same
                // stack as the kernel provided to us.
                "br x0;\n",
            )
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm64_entry_custom_begin {
        // Denote start of function with a branch-target-identification for
        // call instructions. Note that the instruction is only valid since
        // ARMv8.5-A (even though the resulting opcode is a no-op on older
        // machines). We require new toolchains, but can run on older
        // hardware just fine.
        ($_:expr) => { "bti c;\n" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_arm64_entry_custom_end {
        // No arch-specific entry point footer.
        ($_:expr) => { "" }
    }

    pub use arch_arm64_asm_prefix as asm_prefix;
    pub use arch_arm64_entry_align as entry_align;
    pub use arch_arm64_entry_code as entry_code;
    pub use arch_arm64_entry_custom_begin as entry_custom_begin;
    pub use arch_arm64_entry_custom_end as entry_custom_end;
}

/// RISC-V 64-bit Architecture Support
///
/// This module implements the required macros and interfaces for the
/// RISC-V architecture with 64-bit addresses.
pub mod riscv64 {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_riscv64_asm_prefix {
        // The '%' is used to denote GNU-AS identifiers for RISC-V.
        ($id:literal) => { concat!("%", $id) }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_riscv64_entry_align {
        // Use 16-byte aligned function entry-points.
        () => { 16 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_riscv64_entry_code {
        ($_:expr) => {
            core::concat!(
                // Mark the IP as undefined from here on, effectively
                // marking this as the last frame for unwinding.
                ".cfi_undefined ra;\n",

                // Call the loader with the stack-pointer as only
                // argument (in %a0). The loader will return the
                // application entry-point in %a0.
                "mv a0, sp;\n",
                "call {0};\n",

                // Jump to the application entry-point with the same
                // stack as the kernel provided to us.
                "jr a0;\n",
            )
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_riscv64_entry_custom_begin {
        // No arch-specific entry point header.
        ($_:expr) => { "" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_riscv64_entry_custom_end {
        // No arch-specific entry point footer.
        ($_:expr) => { "" }
    }

    pub use arch_riscv64_asm_prefix as asm_prefix;
    pub use arch_riscv64_entry_align as entry_align;
    pub use arch_riscv64_entry_code as entry_code;
    pub use arch_riscv64_entry_custom_begin as entry_custom_begin;
    pub use arch_riscv64_entry_custom_end as entry_custom_end;
}

/// Intel 32-bit (x86 / i686) Architecture Support
///
/// This module implements the required macros and interfaces for the
/// x86 architecture with i686 as baseline requirement.
pub mod x86 {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_asm_prefix {
        // The '@' is used to denote GNU-AS identifiers for x86.
        ($id:literal) => { concat!("@", $id) }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_entry_align {
        // Use 16-byte aligned function entry-points.
        () => { 16 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_entry_code {
        ($_:expr) => {
            core::concat!(
                // Mark the IP as undefined from here on, effectively
                // marking this as the last frame for unwinding.
                ".cfi_undefined eip;\n",

                // Call the loader with the stack-pointer as only
                // argument (on stack). The loader will return the
                // application entry-point in %eax.
                // On linux a 16-byte aligned stack is expected on
                // function entry, so bump the SP accordingly (this
                // was introduced by gcc-4.5). Note that the SP is
                // 16-byte aligned when we are called.
                "mov eax, esp;\n",
                "sub esp, 12;\n",
                "push eax;\n",
                "call {0};\n",
                "add esp, 16;\n",

                // Jump to the application entry-point with the same
                // stack as the kernel provided to us.
                "jmp eax;\n",
            )
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_entry_custom_begin {
        // No arch-specific entry point header.
        ($_:expr) => { "" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_entry_custom_end {
        // No arch-specific entry point footer.
        ($_:expr) => { "" }
    }

    pub use arch_x86_asm_prefix as asm_prefix;
    pub use arch_x86_entry_align as entry_align;
    pub use arch_x86_entry_code as entry_code;
    pub use arch_x86_entry_custom_begin as entry_custom_begin;
    pub use arch_x86_entry_custom_end as entry_custom_end;
}

/// Intel 64-bit (x86-64 / amd64) Architecture Support
///
/// This module implements the required macros and interfaces for the
/// x86-64 architecture with no special requirements.
pub mod x86_64 {
    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_64_asm_prefix {
        // The '@' is used to denote GNU-AS identifiers for x86-64.
        ($id:literal) => { concat!("@", $id) }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_64_entry_align {
        // Use 16-byte aligned function entry-points.
        () => { 16 }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_64_entry_code {
        ($_:expr) => {
            core::concat!(
                // Mark the IP as undefined from here on, effectively
                // marking this as the last frame for unwinding.
                ".cfi_undefined rip;\n",

                // Call the loader with the stack-pointer as only
                // argument (in %rdi). The loader will return the
                // application entry-point in %rax.
                "mov rdi, rsp;\n",
                "call {0};\n",

                // Jump to the application entry-point with the same
                // stack as the kernel provided to us.
                "jmp rax;\n",
            )
        }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_64_entry_custom_begin {
        // No arch-specific entry point header.
        ($_:expr) => { "" }
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! arch_x86_64_entry_custom_end {
        // No arch-specific entry point footer.
        ($_:expr) => { "" }
    }

    pub use arch_x86_64_asm_prefix as asm_prefix;
    pub use arch_x86_64_entry_align as entry_align;
    pub use arch_x86_64_entry_code as entry_code;
    pub use arch_x86_64_entry_custom_begin as entry_custom_begin;
    pub use arch_x86_64_entry_custom_end as entry_custom_end;
}

/// Native Architecture
///
/// The architecture native to the compilation target is aliased as
/// `native`. Use it if architecture dependent handling is not
/// required. Note that you should only access symbols available on
/// all architectures through that alias (see the documentation of
/// each symbol).
#[cfg(doc)]
pub use doc as native;

#[cfg(all(not(doc), target_arch = "arm"))]
pub use arm as native;
#[cfg(all(not(doc), target_arch = "aarch64"))]
pub use arm64 as native;
#[cfg(all(not(doc), target_arch = "riscv64"))]
pub use riscv64 as native;
#[cfg(all(not(doc), target_arch = "x86"))]
pub use x86 as native;
#[cfg(all(not(doc), target_arch = "x86_64"))]
pub use x86_64 as native;
