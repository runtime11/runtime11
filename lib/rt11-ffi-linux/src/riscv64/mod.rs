//! Architecture Definitions for RISC-V-64
//!
//! This module provides the linux-kernel API definitions specific
//! to RISC-V-64.

pub mod syscall;

pub use crate::common::errno as errno;
pub use crate::common::nr as nr;
