//! Code for managing CNTP_CTL (*Counter-timer Physical Timer Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTP_CTL (*Counter-timer Physical Timer Control Register*)
pub struct CntpCtl(pub u32);
impl SysReg for CntpCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for CntpCtl {}
impl CntpCtl {
    #[inline]
    /// Reads CNTP_CTL (*Counter-timer Physical Timer Control Register*)
    pub fn read() -> CntpCtl {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CntpCtl {}
impl CntpCtl {
    #[inline]
    /// Writes CNTP_CTL (*Counter-timer Physical Timer Control Register*)
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
