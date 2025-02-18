//! Code for managing PRLAR (*Protection Region Limit Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRLAR (*Protection Region Limit Address Register*)
pub struct Prlar(pub u32);
impl SysReg for Prlar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Prlar {}
impl Prlar {
    #[inline]
    /// Reads PRLAR (*Protection Region Limit Address Register*)
    pub fn read() -> Prlar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prlar {}
impl Prlar {
    #[inline]
    /// Writes PRLAR (*Protection Region Limit Address Register*)
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
