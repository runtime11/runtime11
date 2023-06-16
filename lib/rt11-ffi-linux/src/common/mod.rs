//! Common Interfaces across Architectures
//!
//! This module provides definitions of linux kernel interfaces that are the
//! same on more than one architecture. The different architecture modules can
//! alias their definitions to the ones provided here, in case the architecture
//! definitions would match these. The upstream linux kernel equivalent is the
//! `asm-generic` interface.
//!
//! In most cases, there is no need to ever access this module outside of the
//! architecture definitions.
//!
//! Note that for documentational purposes, this module also exposes some
//! definitions that are not used by any architecture.

pub mod errno;

/// System Call Numbers
///
/// For most architectures, each system is assigned a number, which is used to
/// identify the system call when invoking it. This module exposes these
/// numbers based on the mnemonic of the system call.
///
/// These definitions are auto-generated from linux kernel sources. See the
/// upstream definitions for details.
pub mod nr;

/// Syscall Invocation Trait
///
/// There are different ways to invoke system calls for different platforms.
/// Most have some instruction to call into the kernel, but on other platforms
/// the VDSO is the better option. However, in most situations the signature
/// of those invocations are the same. This trait provides the entry-points
/// for invoking system calls, but leaves it up to the architecture modules
/// to implement the trait.
///
/// In most cases, the trait is implemented on a dummy structure that has no
/// associated context. But for VDSO invocation, or syscall-redirection, a
/// more elaborate context can be used.
///
/// Note that this can only be used to access the native platform. It relies
/// on `usize` to match the native platform. Fortunately, there is little
/// purpose in using this trait for foreign access, anyway.
pub trait Syscall {
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        self.syscall6(nr, 0, 0, 0, 0, 0, 0)
    }

    unsafe fn syscall1(
        &self,
        nr: usize,
        arg0: usize,
    ) -> usize {
        self.syscall6(nr, arg0, 0, 0, 0, 0, 0)
    }

    unsafe fn syscall2(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
    ) -> usize {
        self.syscall6(nr, arg0, arg1, 0, 0, 0, 0)
    }

    unsafe fn syscall3(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
    ) -> usize {
        self.syscall6(nr, arg0, arg1, arg2, 0, 0, 0)
    }

    unsafe fn syscall4(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    ) -> usize {
        self.syscall6(nr, arg0, arg1, arg2, arg3, 0, 0)
    }

    unsafe fn syscall5(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize {
        self.syscall6(nr, arg0, arg1, arg2, arg3, arg4, 0)
    }

    unsafe fn syscall6(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;
}
