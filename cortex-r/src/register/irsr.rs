//! Code for managing IRSR (*Instruction Region Size and Enable Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// IRSR (*Instruction Region Size and Enable Register*)
pub struct Irsr(pub u32);
impl SysReg for Irsr {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 3;
}
impl crate::register::SysRegRead for Irsr {}
impl Irsr {
    #[inline]
    /// Reads IRSR (*Instruction Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this reads.
    pub fn read() -> Irsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}

impl crate::register::SysRegWrite for Irsr {}
impl Irsr {
    #[inline]
    /// Writes IRSR (*Instruction Region Size and Enable Register*)
    ///
    /// Set RGNR to control which region this affects.
    pub fn write(value: Irsr) {
        unsafe { <Self as SysRegWrite>::write_raw(value.0) }
    }
}
