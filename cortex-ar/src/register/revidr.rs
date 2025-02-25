//! Code for managing REVIDR (*Revision ID Register*)

use crate::register::{SysReg, SysRegRead};

/// REVIDR (*Revision ID Register*)
pub struct Revidr(pub u32);
impl SysReg for Revidr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 6;
}
impl crate::register::SysRegRead for Revidr {}
impl Revidr {
    #[inline]
    /// Reads REVIDR (*Revision ID Register*)
    pub fn read() -> Revidr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
