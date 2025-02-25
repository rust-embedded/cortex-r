//! Code for managing IFSR (*Instruction Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IFSR (*Instruction Fault Status Register*)
pub struct Ifsr(pub u32);
impl SysReg for Ifsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Ifsr {}
impl Ifsr {
    #[inline]
    /// Reads IFSR (*Instruction Fault Status Register*)
    pub fn read() -> Ifsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Ifsr {}
impl Ifsr {
    #[inline]
    /// Writes IFSR (*Instruction Fault Status Register*)
    ///
    /// # Safety
    ///
    /// Ensure that this value is appropriate for this register
    pub unsafe fn write(value: Self) {
        unsafe {
            <Self as SysRegWrite>::write_raw(value.0);
        }
    }
}
