//! Code for managing MPUIR (*MPU Type Register*)

use crate::register::{SysReg, SysRegRead};

/// MPUIR (*MPU Type Register*)
pub struct Mpuir(pub u32);
impl SysReg for Mpuir {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Mpuir {}
impl Mpuir {
    #[inline]
    /// Reads MPUIR (*MPU Type Register*)
    pub fn read() -> Mpuir {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
