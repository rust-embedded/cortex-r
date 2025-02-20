//! Code for managing IRACR (*Instruction Region Access Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IRACR (*Instruction Region Access Control Register*)
pub struct Iracr(pub u32);
impl SysReg for Iracr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 5;
}
impl crate::register::SysRegRead for Iracr {}
impl Iracr {
    #[inline]
    /// Reads IRACR (*Instruction Region Access Control Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Iracr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Iracr {}
impl Iracr {
    #[inline]
    /// Writes IRACR (*Instruction Region Access Control Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Iracr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
