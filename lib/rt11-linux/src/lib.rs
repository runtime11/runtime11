//! Capability-based Linux Kernel Interface Abstractions
//!
//! This module provides platform-independent abstractions of the linux kernel
//! interfaces in native rust following a capability-based model.

#![no_std]

#[cfg(test)]
extern crate std;

pub mod syscall;
pub mod this;
