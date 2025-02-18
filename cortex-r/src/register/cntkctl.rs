//! Code for managing CNTKCTL (*Counter-timer Kernel Control Register*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTKCTL (*Counter-timer Kernel Control Register*)
pub struct Cntkctl(pub u32);
impl SysReg for Cntkctl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 0;
    const CRM: u32 = 1;
    const OP2: u32 = 0;
}
impl crate::register::SysRegRead for Cntkctl {}
impl Cntkctl {
    #[inline]
    /// Reads CNTKCTL (*Counter-timer Kernel Control Register*)
    pub fn read() -> Cntkctl {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for Cntkctl {}
impl Cntkctl {
    #[inline]
    /// Writes CNTKCTL (*Counter-timer Kernel Control Register*)
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
