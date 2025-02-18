//! Code for managing PRBAR (*Protection Region Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR (*Protection Region Base Address Register*)
pub struct Prbar(pub u32);
impl SysReg for Prbar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Prbar {}
impl Prbar {
    #[inline]
    /// Reads PRBAR (*Protection Region Base Address Register*)
    pub fn read() -> Prbar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar {}
impl Prbar {
    #[inline]
    /// Writes PRBAR (*Protection Region Base Address Register*)
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
