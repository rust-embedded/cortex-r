//! Code for managing CNTHCTL (*Counter-timer Hyp Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHCTL (*Counter-timer Hyp Control Register*)
pub struct Cnthctl(pub u32);
impl SysReg for Cnthctl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Cnthctl {}
impl Cnthctl {
    #[inline]
    /// Reads CNTHCTL (*Counter-timer Hyp Control Register*)
    pub fn read() -> Cnthctl {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Cnthctl {}
impl Cnthctl {
    #[inline]
    /// Writes CNTHCTL (*Counter-timer Hyp Control Register*)
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
