//! Access registers in Armv7-R and Armv8-R

pub mod cpsr;
#[doc(inline)]
pub use cpsr::Cpsr;

mod midr;
#[doc(inline)]
pub use midr::Midr;

mod sctlr;
#[doc(inline)]
pub use sctlr::Sctlr;

#[cfg(arm_architecture = "v8-r")]
mod armv8r;
#[doc(inline)]
#[cfg(arm_architecture = "v8-r")]
pub use armv8r::*;

/// Describes a System Register
trait SysReg {
    /// Which Co-Processor (e.g. 15 for CP15) is this register in?
    const CP: u32;
    /// Which CRn argument (e.g. 0 for c0) accesses this register
    const CRN: u32;
    /// Which OP1 argument accesses this register
    const OP1: u32;
    /// Which CRm argument (e.g. 0 for c0) accesses this register
    const CRM: u32;
    /// Which OP2 argument accesses this register
    const OP2: u32;
}

/// Readable System Registers
trait SysRegRead: SysReg {
    #[inline]
    unsafe fn read_raw() -> u32 {
        let r: u32;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mrc p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = out(reg) r,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        r
    }
}

/// Writable System Registers
trait SysRegWrite: SysReg {
    #[inline]
    unsafe fn write_raw(_value: u32) {
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!(
                "mcr p{cp}, {op1}, {reg}, c{crn}, c{crm}, {op2}",
                cp = const Self::CP,
                op1 = const Self::OP1,
                reg = in(reg) _value,
                crn = const Self::CRN,
                crm = const Self::CRM,
                op2 = const Self::OP2,
                options(nomem, nostack, preserves_flags)
            );
        }
    }
}

// TODO:

// Multiprocessor Affinity Register (MPIDR)

// System Control Register

// Auxilliary Control Register

// Coprocessor Access Control Register

// Data Fault Status Register

// Instruction Fault Status Register

// Data Fault Address Register

// Instruction Fault Address Register

// MPU Region Base Address Register

// MPU Region Size and Enable Register

// MPU Region Access Control Register

// MPU Region Number Register

// Context ID Register

// Software Thread ID Register

// Configuration Base Address Register
