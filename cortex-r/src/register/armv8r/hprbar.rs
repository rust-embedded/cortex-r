//! Code for managing HPRBAR (*Hyp Protection Region Base Address Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// HPRBAR (*Hyp Protection Region Base Address Register*)
pub struct Hprbar(pub u32);
impl SysReg for Hprbar {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 4;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Hprbar {}
impl Hprbar {
    #[inline]
    /// Reads HPRBAR (*Hyp Protection Region Base Address Register*)
    pub fn read() -> Hprbar {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Hprbar {}
impl Hprbar {
    #[inline]
    /// Writes HPRBAR (*Hyp Protection Region Base Address Register*)
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
