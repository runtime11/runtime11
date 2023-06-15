//! System Calls on ARM
//!
//! This implements the syscall entries for ARM.
//!
//! The implementation uses the arm-`svc #0` instruction to enter the
//! kernel via a supervisor-call, as it is the recommended way to enter the
//! linux kernel on ARM.
//!
//! Arguments are passed as:
//!     Nr: r7
//!     Args: r0, r1, r2, r3, r4, r5
//! Return value is in:
//!     Ret: r0
//! Always clobbered:
//!     <none>

/// System Call Invocation
///
/// This is a dummy type to invoke standard system calls on ARM via the
/// `svc #0` instruction. The `Syscall` trait is implemented for this dummy.
/// This is the recommended way to invoke system calls on ARM.
///
/// This object can be instantiated by the caller. It is an empty struct and
/// will never carry any information.
pub struct Syscall {}

#[cfg(target_arch = "arm")]
impl crate::common::Syscall for Syscall {
    #[inline]
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "svc 0",
            in("r7") nr,
            lateout("r0") r,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
            in("r1") arg1,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
            in("r1") arg1,
            in("r2") arg2,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
            in("r1") arg1,
            in("r2") arg2,
            in("r3") arg3,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
            in("r1") arg1,
            in("r2") arg2,
            in("r3") arg3,
            in("r4") arg4,
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
            in("r7") nr,
            inlateout("r0") arg0 => r,
            in("r1") arg1,
            in("r2") arg2,
            in("r3") arg3,
            in("r4") arg4,
            in("r5") arg5,
            options(nostack, preserves_flags)
        );

        r
    }
}
