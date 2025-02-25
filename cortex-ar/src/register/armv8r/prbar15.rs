//! Code for managing PRBAR15 (*Protection Region Base Address Register 15*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// PRBAR15 (*Protection Region Base Address Register 15*)
pub struct Prbar15(pub u32);
impl SysReg for Prbar15 {
    const CP: u32 = 15;
    const CRN: u32 = 6;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 4;
}
impl crate::register::SysRegRead for Prbar15 {}
impl Prbar15 {
    #[inline]
    /// Reads PRBAR15 (*Protection Region Base Address Register 15*)
    pub fn read() -> Prbar15 {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Prbar15 {}
impl Prbar15 {
    #[inline]
    /// Writes PRBAR15 (*Protection Region Base Address Register 15*)
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
