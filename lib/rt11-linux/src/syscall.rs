//! Syscall Invocation
//!
//! This module provides symbols for all available system calls, implementing a
//! uniform API to call into the kernel. Any architecture-peculiarities are
//! hidden from the caller, except if they leak into external data definitions.
//! That is, binary formatting of argument structures still needs to be
//! performed by the caller. However, correct syscall invocation and splitting
//! across registers is performed by these helpers.
//!
//! Note that the kernel is particularly good in using any unused bits in input
//! and output values to assign new meaning to. Therefore, while we try to be
//! as specific in the type-system as possible, we must also make sure to be
//! future-proof and allow passing invalid values along just as well.

use rt11_ffi_linux;

/// Error Number
///
/// The linux kernel commonly returns error information as an integer code
/// between 1 and 4096. These have associated symbolic names and are used each
/// for a wide range of possible errors, some more specific, some more generic.
///
/// We encode the error numbers as a u16, to better encapsulate their range.
/// This can be easily converted to the `i32` used by most C standard
/// libraries.
///
/// A value of 0 is not a valid error number, same as any value greater than
/// 4096. It depends on the context how these invalid values are treated.
pub type Errno = u16;

/// Return syscall result to rust
///
/// Take a `usize` return value of a linux system call and convert it to the
/// idiomatic `Result<usize, Errno>`. Almost all system calls treat the
/// return value as a signed integer, with `-1` to `-4096` as error codes.
/// Any other values are considered success. This encoding is binary
/// compatible to pointer values on all architectures, since the upper pages
/// are occupied by the kernel pages, anyway.
///
/// Only a small range of system calls does not adhere to this scheme. Those
/// calls need to use other methods to return the information.
///
/// This function turns any valid error code into `Err<Errno>`, but leaves
/// everything else untouched as `Ok<usize>`.
pub fn result_from_retval(r: usize) -> Result<usize, Errno> {
    if r > core::usize::MAX - 4096 {
        Err((!r + 1) as u16)
    } else {
        Ok(r)
    }
}

/// Syscall Invocation
///
/// This type represents necessary context to invoke system calls. Since most
/// platforms allow invoking system calls without requiring any context, this
/// type is mostly a no-op.
///
/// On some systems, however, system calls are preferably dispatched through
/// the VDSO and thus a context is needed for better syscall performance.
pub struct Syscall {
    ffi: rt11_ffi_linux::native::syscall::Syscall,
}

impl Syscall {
    /// Create new instance
    ///
    /// Create a new instance of the syscall dispatcher. By default, it uses
    /// the fallback mechanism of each platform which requires no special
    /// context to invoke system calls.
    pub(crate) fn new() -> Syscall {
        Self {
            ffi: rt11_ffi_linux::native::syscall::Syscall {},
        }
    }

    /// Close File Descriptor
    ///
    /// `fn sys_close(fd: u32) -> i32`
    ///
    /// Close the file-descriptor specified by the first argument. First, the
    /// file-descriptor is unlinked from the file-descriptor table of the
    /// calling task, then the reference count of the open file-description is
    /// decremented and possibly released thereafter.
    ///
    /// This system call always unlinks the file-descriptor from the
    /// file-descriptor table of the calling task. That is, if the passed
    /// file-descriptor is valid, it is always invalidated by this system call,
    /// regardless of the return code, even if `EINTR` is returned. You must
    /// never repeat or restart this system call.
    ///
    /// Takes a single argument `fd` which specifies the file-descriptor to
    /// close. Unlike most other system calls, this type is `unsigned`, but
    /// that should make no observable difference to the caller. Even if you
    /// pass a negative value, it will exceed the highest allowed index in the
    /// kernel and thus yield `EBADF` from this system call.
    ///
    /// This system call returns `EBADF` if the specified file-descriptor was
    /// invalid. In this case, this system call was a no-op. In all other
    /// cases, regardless of the return code, the system call actually closed
    /// the file-descriptor. Moreover, if this did not release the underlying
    /// open file-description, then this will always return 0. However, if this
    /// system call ends up releasing the underlying open file-description, the
    /// teardown operation of just this can trigger any kind of writeback,
    /// cache-invalidation, resource relinking, rcu grace period, etc., and
    /// thus might take a considerable amount of time. Furthermore, for
    /// historical reasons, this final teardown can also return arbitrary error
    /// codes from deep down in the kernel device drivers (even confusingly
    /// allowing `EBADF`). Given that, you should never check the return value
    /// of this system call, but always assume it succeeded.
    ///
    /// Lastly, you must never assume that a call to this operation actually
    /// performs a final teardown of the underlying open file-description. Any
    /// temporary, parallel kernel maintenance thread might pin the same open
    /// file-description for a short moment, and thus delay the teardown for an
    /// arbitrary amount of time. However, under all circumstances, this will
    /// trigger a `FLUSH` operation on the underlying device (which itself is
    /// highly dependent on the type of device).
    pub unsafe fn close(&self, fd: u32) -> Result<usize, Errno> {
        result_from_retval(
            unsafe {
                <_ as rt11_ffi_linux::common::Syscall>::syscall1(
                    &self.ffi,
                    rt11_ffi_linux::native::nr::CLOSE as usize,
                    fd as usize,
                )
            }
        )
    }

    /// Exit Task
    ///
    /// Stop the current execution and tear down this task. Other tasks of a
    /// possible thread group are left around. See the linux task model for
    /// information how threads and processes map to linux tasks.
    ///
    /// Takes a single argument `code` which specifies the exit condition of
    /// the running task.
    ///
    /// This system call never returns, under no circumstances. This also
    /// implies that this system call cannot be interrupted.
    ///
    /// The kernel uses the lower byte of `code` as exit-code of the task. The
    /// remaining bits of `code` are ignored.
    pub fn exit(&self, code: u32) -> ! {
        let r = unsafe {
            <_ as rt11_ffi_linux::common::Syscall>::syscall1(
                &self.ffi,
                rt11_ffi_linux::native::nr::EXIT as usize,
                code as usize,
            )
        };
        core::unreachable!("`syscall(EXIT)` returned unexpectedly: {}", r);
    }

    /// Restart System Call
    ///
    /// This system call continues an interrupted system call with the same
    /// parameters it was initially called, adjusted only for the time
    /// difference between the original syscall and now.
    ///
    /// This system call is used by the kernel itself to resume system calls of
    /// frozen processes. Whenever a system call is interrupted, the kernel
    /// saves the system call parameters and restarts the system call with the
    /// same parameters once the task is resumed again. However, for system
    /// calls that take relative time-frames as arguments, the kernel usually
    /// needs to adjust these relative time-frames for the elapsed time. For
    /// those system calls, the kernel refrains from restarting the system call
    /// directly and instead changes the system call number of the
    /// to-be-restarted call to this system call. When this system call is then
    /// invoked, the kernel fetches the original system call and its parameters
    /// from its internal state, adjusts the relative timeout, and then
    /// restarts the original system call.
    ///
    /// There is usually no reason why you would ever invoke this system call
    /// from user-space. Moreover, even when the kernel triggers a syscall
    /// restart with this system call, it never leaves kernel space, and thus
    /// user-space should never see this system call at all. Tracing debuggers
    /// might see it, though. And they are the only ones that might reasonable
    /// interfere with it.
    ///
    /// If no system call is to be resumed, this system call returns `EINTR`.
    /// Otherwise, it resumes the original system call with adjusted relative
    /// time parameters and returns the result of the resumed system call.
    pub unsafe fn restart_syscall(&self) -> Result<usize, Errno> {
        result_from_retval(
            unsafe {
                <_ as rt11_ffi_linux::common::Syscall>::syscall0(
                    &self.ffi,
                    rt11_ffi_linux::native::nr::RESTART_SYSCALL as usize,
                )
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Verify `result_from_retval()`. Check that error codes are correctly
    // detected as such.
    #[test]
    fn retval_check() {
        let success_values = [
            0, 1, 2, 3,
            254, 255, 256, 257,
            65534, 65535, 65536, 65537,
            core::usize::MAX / 2,
            core::usize::MAX / 2 + 1,
            core::usize::MAX - 4097,
            core::usize::MAX - 4096,
        ];

        for v in &success_values {
            let r = result_from_retval(*v);
            assert_eq!(r, Ok(*v));
        }

        let error_values = [
            (4096, core::usize::MAX - 4095),
            (4095, core::usize::MAX - 4094),
            (4094, core::usize::MAX - 4093),
            (4093, core::usize::MAX - 4092),
            (4, core::usize::MAX - 3),
            (3, core::usize::MAX - 2),
            (2, core::usize::MAX - 1),
            (1, core::usize::MAX),
        ];

        for (c, v) in &error_values {
            let r = result_from_retval(*v);
            assert_eq!(r, Err(*c));
        }
    }

    // Verify that `Syscall` instances can be created without context.
    #[test]
    fn syscall_creation() {
        let _: Syscall = Syscall::new();
    }
}
