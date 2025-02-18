//! Code for managing CNTP_TVAL (*Counter-timer Physical Timer RimerValue Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTP_TVAL (*Counter-timer Physical Timer RimerValue Register*)
pub struct CntpTval(pub u32);
impl SysReg for CntpTval {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for CntpTval {}
impl CntpTval {
    #[inline]
    /// Reads CNTP_TVAL (*Counter-timer Physical Timer RimerValue Register*)
    pub fn read() -> CntpTval {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CntpTval {}
impl CntpTval {
    #[inline]
    /// Writes CNTP_TVAL (*Counter-timer Physical Timer RimerValue Register*)
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
