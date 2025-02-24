//! Code for managing HPRLAR (*Hyp Protection Region Limit Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRLAR (*Hyp Protection Region Limit Address Register*)
pub struct Hprlar(pub u32);
impl SysReg for Hprlar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for Hprlar {}
impl Hprlar {
    #[inline]
    /// Reads HPRLAR (*Hyp Protection Region Limit Address Register*)
    pub fn read() -> Hprlar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprlar {}
impl Hprlar {
    #[inline]
    /// Writes HPRLAR (*Hyp Protection Region Limit Address Register*)
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
