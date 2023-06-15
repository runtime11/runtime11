//! System Calls on ARM-64
//!
//! This implements the syscall entries for ARM-64.
//!
//! The implementation uses the aarch64-`svc #0` instruction to enter the
//! kernel via a supervisor-call, as it is the recommended way to enter the
//! linux kernel on ARM-64.
//!
//! Arguments are passed as:
//!     Nr: x8 (more precicely `w8`, the bottom half of `x8`)
//!     Args: x0, x1, x2, x3, x4, x5
//! Return value is in:
//!     Ret: x0
//! Always clobbered:
//!     <none>

/// System Call Invocation
///
/// This is a dummy type to invoke standard system calls on ARM-64 via the
/// `svc #0` instruction. The `Syscall` trait is implemented for this dummy.
/// This is the recommended way to invoke system calls on ARM-64.
///
/// This object can be instantiated by the caller. It is an empty struct and
/// will never carry any information.
pub struct Syscall {}

#[cfg(target_arch = "aarch64")]
impl crate::common::Syscall for Syscall {
    #[inline]
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "svc 0",
            in("x8") nr,
            lateout("x0") r,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
            in("x1") arg1,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
            in("x1") arg1,
            in("x2") arg2,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            in("x4") arg4,
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
            "svc 0",
            in("x8") nr,
            inlateout("x0") arg0 => r,
            in("x1") arg1,
            in("x2") arg2,
            in("x3") arg3,
            in("x4") arg4,
            in("x5") arg5,
            options(nostack, preserves_flags)
        );

        r
    }
}
