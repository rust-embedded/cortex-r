//! Code for managing HSCTLR (*Hyp System Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HSCTLR (*Hyp System Control Register*)
pub struct Hsctlr(pub u32);
impl SysReg for Hsctlr {
    const CP: u32 = 15;
    const CRN: u32 = 1;
    const OP1: u32 = 4;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hsctlr {}
impl Hsctlr {
    #[inline]
    /// Reads HSCTLR (*Hyp System Control Register*)
    pub fn read() -> Hsctlr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hsctlr {}
impl Hsctlr {
    #[inline]
    /// Writes HSCTLR (*Hyp System Control Register*)
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
