//! Current Execution Context
//!
//! On linux, every thread of execution carries implicit context. In the kernel
//! these are represented by `struct task_struct` and commonly called `Task`. A
//! task represents a single execution context, which most closely resembles
//! what higher abstractions call `Thread`. However, the task model does not
//! necessarily share semantics with the common thread model. The linux kernel
//! allows fine-grained control over which resources are shared between tasks,
//! and which are not. It can be used to implement a thread model according to
//! POSIX rules, but it can also be used to create completely different models
//! where some resources are shared but others are not.
//!
//! The resources pinned by a task are implicit arguments to every kernel
//! interface. This is hardly compatible to capability-based models. Therefore,
//! this module provides an abstraction used to represent the current execution
//! context, called `This`.

/// Current Execution Context
///
/// This type is used to encapsulate the implicit kernel context for the
/// current thread of execution. Any operation that implicitly uses information
/// from the current context, or operates on it, is instead provided as a
/// method on this type. This explicitly documents this behaver and hard-codes
/// the dependency.
///
/// Note that this type is explicitly marked non-`Send` to prevent it being
/// used outside its owning execution. This type is explicitly not meant to
/// represent tasks outside of their own execution and thus must not be
/// access from outside the calling task.
///
/// Only a single instance of this type can exist for each task. It is the
/// responsibility of the creator of the task to create the initial instance.
pub struct This {
    pub syscall: crate::syscall::Syscall,
    _marker_nonsend: core::marker::PhantomData<*mut ()>,
}

impl This {
    /// Create a new execution context
    ///
    /// Safety
    /// ------
    ///
    /// Only a single execution context per task may exist. The caller must
    /// ensure that no other context exists.
    pub unsafe fn new() -> This {
        Self {
            syscall: crate::syscall::Syscall::new(),
            _marker_nonsend: core::default::Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Verify that `This` instances can be created without context.
    #[test]
    fn this_creation() {
        let _: This = unsafe { This::new() };
    }

    // Verify that the syscall invocation via `This` is accessible. We simply
    // call `close(2)` on `-1` and verify that we get `EBADF` back.
    #[test]
    fn this_syscall() {
        let this: This = unsafe { This::new() };

        unsafe {
            assert_eq!(
                this.syscall.close(-1i32 as u32),
                Err(rt11_ffi_linux::native::errno::EBADF),
            );
        }
    }
}
