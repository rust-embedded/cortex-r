//! Code for managing CNTHP_CTL (*Counter-timer Hyp Physical Timer Control Register (EL2)*)

use crate::register::{SysReg, SysRegRead, SysRegWrite};

/// CNTHP_CTL (*Counter-timer Hyp Physical Timer Control Register (EL2)*)
pub struct CnthpCtl(pub u32);
impl SysReg for CnthpCtl {
    const CP: u32 = 15;
    const CRN: u32 = 14;
    const OP1: u32 = 4;
    const CRM: u32 = 2;
    const OP2: u32 = 1;
}
impl crate::register::SysRegRead for CnthpCtl {}
impl CnthpCtl {
    #[inline]
    /// Reads CNTHP_CTL (*Counter-timer Hyp Physical Timer Control Register (EL2)*)
    pub fn read() -> CnthpCtl {
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }
}
impl crate::register::SysRegWrite for CnthpCtl {}
impl CnthpCtl {
    #[inline]
    /// Writes CNTHP_CTL (*Counter-timer Hyp Physical Timer Control Register (EL2)*)
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
