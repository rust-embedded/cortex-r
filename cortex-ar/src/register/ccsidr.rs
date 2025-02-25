//! Code for managing CCSIDR (*Current Cache Size ID Register*)

use crate::register::{SysReg, SysRegRead};

/// CCSIDR (*Current Cache Size ID Register*)
pub struct Ccsidr(pub u32);
impl SysReg for Ccsidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 1;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Ccsidr {}
impl Ccsidr {
    #[inline]
    /// Reads CCSIDR (*Current Cache Size ID Register*)
    pub fn read() -> Ccsidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
