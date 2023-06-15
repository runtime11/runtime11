//! System Calls on x86
//!
//! This implements the syscall entries for x86.
//!
//! The implementation uses the x86-`int$0x80` software interrupt to enter the
//! kernel. It would be much faster to use the VDSO entry point, but it does
//! require access to `%gs` and the TLS mappings, and thus is left to higher
//! level crates to implement.
//!
//! Arguments are passed as:
//!     Nr: eax
//!     Args: ebx, ecx, edx, esi, edi, ebp
//! Return value is in:
//!     Ret: eax

/// System Call Invocation
///
/// This is a dummy type to invoke standard system calls on x86 via the
/// `int$0x80` software interrupt. The `Syscall` trait is implemented for
/// this dummy.
///
/// This object can be instantiated by the caller. It is an empty struct and
/// will never carry any information.
pub struct Syscall {}

#[cfg(target_arch = "x86")]
impl crate::common::Syscall for Syscall {
    #[inline]
    unsafe fn syscall0(
        &self,
        nr: usize,
    ) -> usize {
        let mut r: usize;

        core::arch::asm!(
            "int $0x80",
            inlateout("eax") nr => r,
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
            "int $0x80",
            inlateout("eax") nr => r,
            in("ebx") arg0,
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
            "int $0x80",
            inlateout("eax") nr => r,
            in("ebx") arg0,
            in("ecx") arg1,
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
            "int $0x80",
            inlateout("eax") nr => r,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
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

        // LLVM reserves `esi` for inline-asm management (to make sure stack
        // management is not corrupted). However, it is completely save to use
        // `esi`, and it is not clobbered by the kernel. GCC allows using it
        // for inline-asm input, but unfortunately LLVM does not. Hence, we
        // have to manually swap it out with whatever was picked as alternative
        // for arg3.
        //
        // Note that in most cases LLVM still picks `esi`, so this looks
        // slightly stupid running `xchg esi, esi`. Unfortunately, there is
        // little we can do about it, so we keep it as it is. Fortunately, the
        // extra instruction has little performance impact for a system call
        // entry, so it is safe to ignore. Additionally, on x86 you better use
        // VDSO entries, anyway, if you care for optimal performance.
        core::arch::asm!(
            "xchg esi, {arg3}",
            "int $0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") nr => r,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
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

        // see syscall4() for `esi` handling
        core::arch::asm!(
            "xchg esi, {arg3}",
            "int $0x80",
            "xchg esi, {arg3}",
            arg3 = in(reg) arg3,
            inlateout("eax") nr => r,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
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

        // The last argument `arg5` needs to be passed in `ebp`. Again, LLVM
        // does not allow us to use it as `in`-register. Hence, we just let
        // LLVM pick a register itself. Since there a none left, it will pick
        // the right one, anyway. But we try to be safe and assume both `arg3`
        // and `arg5` might be in other registers (or actually swapped). Hence,
        // we just push the values to the stack, then save `esi` and `ebp`,
        // then load the values into those registers and jump into the kernel.
        // Afterwards, we restore `esi` and `ebp` again, and restore the
        // registers picked by LLVM.
        //
        // Note that the assembly will likely look stupid, since `arg3` usually
        // ends up being `esi` and `arg5` ends up being `ebp`. Unfortunately,
        // there is little we can do to detect that scenario. However, a
        // 6-argument syscall is likely not noticing the slight slowdown by
        // this. Furthermore, VDSO entries are recommended on x86 for better
        // performance, anyway.
        core::arch::asm!(
            "push {arg3}",
            "push {arg5}",
            "push esi",
            "push ebp",
            "mov ebp, DWORD PTR [esp + 8]",
            "mov esi, DWORD PTR [esp + 12]",
            "int $0x80",
            "pop ebp",
            "pop esi",
            "pop {arg5}",
            "pop {arg3}",
            arg3 = in(reg) arg3,
            arg5 = in(reg) arg5,
            inlateout("eax") nr => r,
            in("ebx") arg0,
            in("ecx") arg1,
            in("edx") arg2,
            in("edi") arg4,
            options(preserves_flags)
        );

        r
    }
}
