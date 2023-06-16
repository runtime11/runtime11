//! Definitions of the Linux Kernel Interfaces
//!
//! This module exposes the definitions of the linux kernel interfaces
//! transposed to Rust. This allows interacting with the linux kernel from
//! native Rust code. Note that this module merely provides the definitions
//! of the structures and entry-points of the kernel, but no abstractions or
//! rustifications of those interfaces. This is left up to higher level
//! modules.
//!
//! The kernel ABI is highly architecture-dependent. Hence, this module
//! exposes one sub-module for each supported architecture, which then expose
//! the _entire_ set of kernel interfaces for each given architecture. Since
//! a lot of this is shared across architectures, the `common` sub-module
//! provides shared definitions that are then aliases by each architecture
//! that uses them, respectively.
//!
//! The architecture native to the compilation target is aliased as `native`.
//! The APIs exposed by the different architectures do not necessarily match.
//! The kernel APIs have historical differences between architectures, and
//! these are blatantly exposed by this module. Hence, care is required to
//! verify a specific interface is actually available on the required
//! architectures.
//!
//! Note that all architecture module are compiled unconditionally. Only the
//! bits that cannot be compiled on foreign platforms (e.g., inline assembly)
//! are guarded by conditional compilation. This improves compilation-coverage
//! and allows partial testing of foreign platforms. Furthermore, it allows
//! using this module to interact with foreign platforms even at runtime (e.g.,
//! in debug utilities).
//!
//! The linux kernel interfaces are vast and this module aims to be an
//! exhaustive implementation of those definitions. Unfortunately, this work
//! has not concluded, yet. Therefore, if required definitions are missing,
//! report them to the upstream bug-tracker.
//!
//! Transposition Rules
//! -------------------
//!
//! The aim is to transpose the kernel definitions to rust following strict
//! rules to make it easy to find definitions, compare them to the reference
//! definitions in C, and argue about their validity. Unfortunately, the
//! kernel definitions use very lax namespacing, which makes it hard to group
//! them into rust modules. Furthermore, definitions are commonly moved
//! between kernel headers (relying on sub-includes), ruling out
//! modularization based on header names. To avoid putting all definitions
//! into a single module, they are grouped based on the kernel subsystem that
//! ultimately owns them, or the UNIX/POSIX term that commonly describes them.
//! This is quite subjective, yet no better alternative has been found.
//!
//! Furthermore, with more and more definitions being transposed, more rules
//! will have to be defined. The following is a non-exhaustive list of rules
//! that have been followed so far:
//!
//!  * The kernel uses 32-bit `int`, so any use of `int` is transposed as
//!    `i32` / `u32`.
//!
//!  * The kernel uses `long` to store addresses. To allow foreign-architecture
//!    access, these must be transposed to `u32` / `u64` respectively.
//!
//!  * The kernel _often_ uses `long long` to represent 64-bit addresses on
//!    32-bit architectures. In most situations, you want `i64` / `u64` to
//!    represent it.
//!
//!  * Fixed size `__u32`, `__u64`, etc. types use the native rust fixed-sized
//!    integers.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod common;

pub mod arm;
pub mod arm64;
pub mod riscv64;
pub mod x86;
pub mod x86_64;

/// Native Architecture
///
/// The architecture native to the compilation target is aliased as
/// `native`. Use it if architecture dependent handling is not
/// required. Note that you should only access symbols available on
/// all architectures through that alias (see the documentation of
/// each symbol).
///
/// For documentational purposes, the shared `common` architecture is
/// marked as native. This is never the case for real compilation
/// targets, though!
///
/// If the native architecture is not a supported architecture, this
/// alias will not be available.
#[cfg(doc)]
pub use common as native;

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

#[cfg(test)]
mod test {
    use super::*;

    // Verify the `native` module is available and links to an
    // actual architecture.
    #[test]
    fn native_check() {
        assert_ne!(native::nr::EXIT, 0);
    }

    // Verify all architectures are always compiled in and accessible. We
    // simply check for their hard-coded `nr::EXIT` symbols here, which
    // exist everywhere.
    #[test]
    fn arch_availability() {
        assert_eq!(common::nr::EXIT, 93);

        assert_eq!(arm::nr::EXIT, 1);
        assert_eq!(arm64::nr::EXIT, 93);
        assert_eq!(riscv64::nr::EXIT, 93);
        assert_eq!(x86::nr::EXIT, 1);
        assert_eq!(x86_64::nr::EXIT, 60);
    }

    // Check for basic architecture properties that need to be satisfied by
    // all linux architectures and is relied upon.
    #[test]
    fn arch_check() {
        assert!(core::mem::size_of::<usize>() >= 4);
        assert!(core::mem::align_of::<usize>() >= 4);
    }

    // Verify that the dummy `Syscall` type is available for all architectures,
    // required to invoke system calls from process context via standard
    // instructions.
    // Note that the `Syscall` trait is unlikely to be available, since it
    // rarely can be compiled for foreign architectures.
    #[test]
    fn syscall_availability() {
        let _ = arm::syscall::Syscall {};
        let _ = arm64::syscall::Syscall {};
        let _ = riscv64::syscall::Syscall {};
        let _ = x86::syscall::Syscall {};
        let _ = x86_64::syscall::Syscall {};
    }

    // Verify that `common::Syscall` is implemented for the syscall dummy
    // of the native architecture.
    #[test]
    fn syscall_trait() {
        let s6: unsafe fn(
            &native::syscall::Syscall, usize, usize, usize, usize, usize, usize, usize,
        ) -> usize = <native::syscall::Syscall as common::Syscall>::syscall6;

        assert_ne!(s6 as *const () as usize, 0);
    }

    // Run a simple invocation of `syscall0()` and try to see whether it
    // returns plausible information.
    #[test]
    fn syscall_0_check() {
        let sc = native::syscall::Syscall {};

        let r0 = unsafe {
            <_ as common::Syscall>::syscall0(
                &sc,
                native::nr::GETPID as usize,
            )
        };
        assert_eq!(r0, std::process::id() as usize);
    }

    // Run a simple invocation of `syscall1()` and `syscall2()` and see
    // whether they behave plausibly.
    #[test]
    fn syscall_1_2_check() {
        // We run `pipe2()` and verify the `close()` syscall accepts the values
        // without complaint.
        let sc = native::syscall::Syscall {};
        let mut p0: [u32; 2] = [0, 0];

        let r0 = unsafe {
            <_ as common::Syscall>::syscall2(
                &sc,
                native::nr::PIPE2 as usize,
                p0.as_mut_ptr() as usize,
                0,
            )
        };
        assert_eq!(r0, 0);
        assert!(p0[0] > 2);
        assert!(p0[1] > 2);
        assert_ne!(p0[0], p0[1]);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                p0[0] as usize,
            )
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                p0[1] as usize,
            )
        };
        assert_eq!(r0, 0);
    }

    // Run a simple invocation of `syscall3()` and see whether it behaves
    // plausibly.
    #[test]
    fn syscall_3_check() {
        // We create a pipe, write to one end and verify we can read the same
        // data from the other end.
        let sc = native::syscall::Syscall {};
        let mut p0: [u32; 2] = [0, 0];
        let mut b0: [u8; 16] = [0; 16];

        let r0 = unsafe {
            <_ as common::Syscall>::syscall2(
                &sc,
                native::nr::PIPE2 as usize,
                p0.as_mut_ptr() as usize,
                0,
            )
        };
        assert_eq!(r0, 0);
        assert!(p0[0] > 2);
        assert!(p0[1] > 2);
        assert_ne!(p0[0], p0[1]);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall3(
                &sc,
                native::nr::WRITE as usize,
                p0[1] as usize,
                "foobar".as_ptr() as usize,
                6 as usize,
            )
        };
        assert_eq!(r0, 6);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall3(
                &sc,
                native::nr::READ as usize,
                p0[0] as usize,
                b0.as_mut_ptr() as usize,
                6 as usize,
            )
        };
        assert_eq!(r0, 6);
        assert_eq!(core::str::from_utf8(&b0[..6]), Ok("foobar"));

        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                p0[0] as usize,
            )
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                p0[1] as usize,
            )
        };
        assert_eq!(r0, 0);
    }

    // Run a simple `syscall4()` invocation and see whether it behaves
    // plausibly.
    #[test]
    fn syscall_4_check() {
        // We create a memfd and then query `/proc` for the link-value of the
        // memfd. This is ABI and needs to be the (annotated) name that we
        // passed to `memfd_create()`.
        let sc = native::syscall::Syscall {};
        let mut b0: [u8; 128] = [0; 128];

        let f0 = unsafe {
            <_ as common::Syscall>::syscall2(
                &sc,
                native::nr::MEMFD_CREATE as usize,
                "foobar\x00".as_ptr() as usize,
                0,
            )
        };
        assert!(f0 > 2);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall4(
                &sc,
                native::nr::READLINKAT as usize,
                core::usize::MAX - 100 + 1, // AT_FDCWD
                std::format!("/proc/self/fd/{}\x00", f0).as_str().as_ptr() as usize,
                b0.as_mut_ptr() as usize,
                128 - 1,
            )
        };
        assert_eq!(r0, 23);
        assert_eq!(
            core::str::from_utf8(&b0[..23]).unwrap(),
            "/memfd:foobar (deleted)",
        );

        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                f0,
            )
        };
        assert_eq!(r0, 0);
    }

    // Run a simple `syscall5()` invocation and see whether it behaves
    // plausibly.
    #[test]
    fn syscall_5_check() {
        // Run `statx()` on `STDIN`, but pass `AT_SYMLINK_NOFOLLOW`. This
        // means we instead get information on the symlink in `/proc`. Check
        // that this was correctly interpreted by the kernel and verify the
        // `S_IFLNK` flag is set on the result.
        let sc = native::syscall::Syscall {};
        let mut b0: [u32; 1024] = [0; 1024];

        let r0 = unsafe {
            <_ as common::Syscall>::syscall5(
                &sc,
                native::nr::STATX as usize,
                core::usize::MAX - 100 + 1, // AT_FDCWD
                "/proc/self/fd/0".as_ptr() as usize,
                0x100, // AT_SYMLINK_NOFOLLOW
                0x1,   // STATX_TYPE
                b0.as_mut_ptr() as usize,
            )
        };
        assert_eq!(r0, 0);
        assert_ne!(b0[0] & 0x1, 0);
        assert_eq!(
            unsafe {
                core::ptr::read_unaligned(
                    (b0.as_ptr() as *const u16).offset(14)
                )
            } & 0o170000, // S_IFMT
            0o120000, // S_IFLNK
        );
    }

    // Run a simple `syscall6()` invocation and see whether it behaves
    // plausibly.
    #[test]
    fn syscall_6_check() {
        // Create two memfd instances, write a text into the first one. Use
        // the `copy_file_range()` syscall to copy the data over into the other
        // memfd and then verify via `read()`. Use `lseek()` to reset the file
        // position between calls.

        let sc = native::syscall::Syscall {};
        let mut b0: [u8; 128] = [0; 128];

        let f0 = unsafe {
            <_ as common::Syscall>::syscall2(
                &sc,
                native::nr::MEMFD_CREATE as usize,
                "foobar\x00".as_ptr() as usize,
                0,
            )
        };
        let f1 = unsafe {
            <_ as common::Syscall>::syscall2(
                &sc,
                native::nr::MEMFD_CREATE as usize,
                "foobar\x00".as_ptr() as usize,
                0,
            )
        };
        assert!(f0 > 2);
        assert!(f1 > 2);
        assert_ne!(f0, f1);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall3(
                &sc,
                native::nr::WRITE as usize,
                f0 as usize,
                "foobar".as_ptr() as usize,
                6 as usize,
            )
        };
        assert_eq!(r0, 6);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall3(
                &sc,
                native::nr::LSEEK as usize,
                f0 as usize,
                0 as usize,
                0 as usize,
            )
        };
        assert_eq!(r0, 0);

        let r0 = unsafe {
            <_ as common::Syscall>::syscall6(
                &sc,
                native::nr::COPY_FILE_RANGE as usize,
                f0 as usize,
                0 as usize,
                f1 as usize,
                0 as usize,
                6 as usize,
                0 as usize,
            )
        };

        // On some architectures, the qemu emulators do not provide wrappers
        // for `copy_file_range()`, so handle `ENOSYS` (-38) properly and skip
        // the test. This is unfortunate and we should probably find another
        // syscall to test that has better coverage.
        if r0 as isize != -(native::errno::ENOSYS as isize) {
            assert_eq!(r0, 6);

            let r0 = unsafe {
                <_ as common::Syscall>::syscall3(
                    &sc,
                    native::nr::LSEEK as usize,
                    f1 as usize,
                    0 as usize,
                    0 as usize,
                )
            };
            assert_eq!(r0, 0);

            let r0 = unsafe {
                <_ as common::Syscall>::syscall3(
                    &sc,
                    native::nr::READ as usize,
                    f1 as usize,
                    b0.as_mut_ptr() as usize,
                    6 as usize,
                )
            };
            assert_eq!(r0, 6);
            assert_eq!(core::str::from_utf8(&b0[..6]), Ok("foobar"));
        }

        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                f1,
            )
        };
        assert_eq!(r0, 0);
        let r0 = unsafe {
            <_ as common::Syscall>::syscall1(
                &sc,
                native::nr::CLOSE as usize,
                f0,
            )
        };
        assert_eq!(r0, 0);
    }
}
