//! Code for managing CSSELR (*Cache Size Selection Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CSSELR (*Cache Size Selection Register*)
pub struct Csselr(pub u32);
impl SysReg for Csselr {
    const CP: u32 = 15;
    const CRN: u32 = 0;
    const OP1: u32 = 2;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Csselr {}
impl Csselr {
    #[inline]
    /// Reads CSSELR (*Cache Size Selection Register*)
    pub fn read() -> Csselr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Csselr {}
impl Csselr {
    #[inline]
    /// Writes CSSELR (*Cache Size Selection Register*)
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
