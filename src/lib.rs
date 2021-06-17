//! # The kernel cpu abstraction library
//! The kernel architecture library foundation of hardware abstract kernel. It
//! is the glue between hardware and high-level kernel abstractions for managing
//! resources. # How to add a new architecture
//!  1. Create a new crate to arch workspace and follow module naming
//! conversation like in other crates from this workspace.
//!  2. Add abstractions to all required functionality from `independent`
//! module.
//!  3. Where it need it manually uses intrinsics for your architecture to
//! implement target-specific things.
#![cfg_attr(not(test), no_std)]
#![cfg_attr(x86_64, feature(abi_x86_interrupt))]

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate lazy_static;

pub mod interrupts;
/// Arch dependent implementations of hal functinality
pub(crate) mod intrinsics;

pub fn low_power_loop() -> ! {
    cfg_if! {
        if #[cfg(riscv)] {
            loop {
                unsafe {
                    arch::riscv::asm::wfi();
                }
            }
        } else if #[cfg(x86_64)] {
            loop {
                arch::x86_64::instructions::hlt();
            }
        } else {
            unimplemented!();
        }
    }
}

pub fn init() {
    crate::interrupts::init();
}
