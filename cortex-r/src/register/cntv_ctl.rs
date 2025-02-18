//! Code for managing CNTV_CTL (*Counter-timer Virtual Timer Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTV_CTL (*Counter-timer Virtual Timer Control Register*)
pub struct CntvCtl(pub u32);
impl SysReg for CntvCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 3;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for CntvCtl {}
impl CntvCtl {
    #[inline]
    /// Reads CNTV_CTL (*Counter-timer Virtual Timer Control Register*)
    pub fn read() -> CntvCtl {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CntvCtl {}
impl CntvCtl {
    #[inline]
    /// Writes CNTV_CTL (*Counter-timer Virtual Timer Control Register*)
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
