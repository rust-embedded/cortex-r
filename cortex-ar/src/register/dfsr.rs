//! Code for managing DFSR (*Data Fault Status Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// DFSR (*Data Fault Status Register*)
pub struct Dfsr(pub u32);
impl SysReg for Dfsr {
    const CP: u32 = 15;
    const CRN: u32 = 5;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Dfsr {}
impl Dfsr {
    #[inline]
    /// Reads DFSR (*Data Fault Status Register*)
    pub fn read() -> Dfsr {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Dfsr {}
impl Dfsr {
    #[inline]
    /// Writes DFSR (*Data Fault Status Register*)
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
