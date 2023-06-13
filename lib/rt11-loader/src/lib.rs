//! ELF Dynamic Loader
//!
//! XXX

#![no_std]

#[cfg(test)]
extern crate std;

pub mod arch;
pub mod entry;

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() {
    }
}
