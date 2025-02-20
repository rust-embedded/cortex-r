//! Code for managing DRACR (*Data Region Access Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DRACR (*Data Region Access Control Register*)
pub struct Dracr(pub u32);
impl SysReg for Dracr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Dracr {}
impl Dracr {
    #[inline]
    /// Reads DRACR (*Data Region Access Control Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Dracr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Dracr {}
impl Dracr {
    #[inline]
    /// Writes DRACR (*Data Region Access Control Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Dracr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
