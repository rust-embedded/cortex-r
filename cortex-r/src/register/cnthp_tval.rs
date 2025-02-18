//! Code for managing CNTHP_TVAL (*Counter-timer Physical Timer TimerValue Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHP_TVAL (*Counter-timer Physical Timer TimerValue Register*)
pub struct CnthpTval(pub u32);
impl SysReg for CnthpTval {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for CnthpTval {}
impl CnthpTval {
    #[inline]
    /// Reads CNTHP_TVAL (*Counter-timer Physical Timer TimerValue Register*)
    pub fn read() -> CnthpTval {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CnthpTval {}
impl CnthpTval {
    #[inline]
    /// Writes CNTHP_TVAL (*Counter-timer Physical Timer TimerValue Register*)
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
