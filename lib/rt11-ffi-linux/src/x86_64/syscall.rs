//! System Calls on x86_64
//!
//! This implements the syscall entries for x86_64.
//!
//! The implementation uses the x86_64-`syscall` instruction to enter the
//! kernel, as it is the recommended way to enter the linux kernel on x86_64.
//!
//! Arguments are passed as:
//!     Nr: rax
//!     Args: rdi, rsi, rdx, r10, r8, r9
//! Return value is in:
//!     Ret: rax
//! Always clobbered:
//!     rcx, r11

/// System Call Invocation
///
/// This is a dummy type to invoke standard system calls on x86 via the
/// `syscall` instruction. The `Syscall` trait is implemented for this dummy.
/// This is the recommended way to invoke system calls on x86_64.
///
/// This object can be instantiated by the caller. It is an empty struct and
/// will never carry any information.
pub struct Syscall {}

#[cfg(target_arch = "x86_64")]
impl crate::common::Syscall for Syscall {
    #[inline]
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "syscall",
            inlateout("rax") nr => r,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            in("rsi") arg1,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            in("r8") arg4,
            out("rcx") _,
            out("r11") _,
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
            "syscall",
            inlateout("rax") nr => r,
            in("rdi") arg0,
            in("rsi") arg1,
            in("rdx") arg2,
            in("r10") arg3,
            in("r8") arg4,
            in("r9") arg5,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );

        r
    }
}
