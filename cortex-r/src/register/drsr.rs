//! Code for managing DRSR (*Data Region Size and Enable Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DRSR (*Data Region Size and Enable Register*)
pub struct Drsr(pub u32);
impl SysReg for Drsr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 2;
}
impl crate::register::SysRegRead for Drsr {}
impl Drsr {
    #[inline]
    /// Reads DRSR (*Data Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Drsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Drsr {}
impl Drsr {
    #[inline]
    /// Writes DRSR (*Data Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Drsr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
