//! System Calls on RISC-V-64
//!
//! This implements the syscall entries for RISC-V-64.
//!
//! The implementation uses the riscv-`ecall` instruction to enter the
//! kernel, as it is the recommended way to enter the linux kernel on
//! RISC-V-64.
//!
//! Arguments are passed as:
//!     Nr: a7
//!     Args: a0, a1, a2, a3, a4, a5
//! Return value is in:
//!     Ret: a0
//! Always clobbered:
//!     <none>

/// System Call Invocation
///
/// This is a dummy type to invoke standard system calls on RISC-V-64 via the
/// `ecall` instruction. The `Syscall` trait is implemented for this dummy.
/// This is the recommended way to invoke system calls on RISC-V-64.
///
/// This object can be instantiated by the caller. It is an empty struct and
/// will never carry any information.
pub struct Syscall {}

#[cfg(target_arch = "riscv64")]
impl crate::common::Syscall for Syscall {
    #[inline]
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            lateout("a0") r,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall1(
        &self,
        nr: usize,
        arg0: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall2(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            in("a1") arg1,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall3(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            in("a1") arg1,
            in("a2") arg2,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall4(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall5(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            options(nostack, preserves_flags)
        );

        r
    }

    #[inline]
    unsafe fn syscall6(
        &self,
        nr: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") arg0 => r,
            in("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            options(nostack, preserves_flags)
        );

        r
    }
}
