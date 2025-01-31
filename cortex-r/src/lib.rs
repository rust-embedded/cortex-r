//! CPU/peripheral support for Arm Cortex-R

#![no_std]

#[cfg(feature = "critical-section-single-core")]
mod critical_section;

pub mod register;

pub mod interrupt;

pub mod asm;

/// Generate an SVC call with the given argument.
///
/// Safe to call even in Supervisor (Svc) mode, as long as your Svc handler
/// saves and restores SPSR_svc correctly.
#[macro_export]
macro_rules! svc {
    ($r0:expr) => {
        unsafe {
            core::arch::asm!("svc {arg}", arg = const $r0, out("lr") _);
        }
    }
}
