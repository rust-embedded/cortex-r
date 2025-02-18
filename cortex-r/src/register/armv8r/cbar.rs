//! Code for the *Configuration Base Address Register*

use crate::register::{SysReg, SysRegRead};

/// The *Configuration Base Address Register* (CBAR)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cbar(u32);

impl SysReg for Cbar {
    const CP: u32 = 15;
    const CRN: u32 = 15;
    const OP1: u32 = 1;
    const CRM: u32 = 3;
    const OP2: u32 = 0;
}

impl SysRegRead for Cbar {}

impl Cbar {
    /// Reads the *Configuration Base Address Register*
    #[inline]
    pub fn read() -> Cbar {
        // Safety: this read has no side-effects
        unsafe { Self(<Self as SysRegRead>::read_raw()) }
    }

    /// Get the periphbase address
    pub fn periphbase(self) -> *mut u32 {
        (self.0 & 0xFFF00000) as *mut u32
    }
}

impl core::fmt::Debug for Cbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CBAR {{ {:010p} }}", self.periphbase())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Cbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CBAR {{ 0x{=usize:08x} }}", self.0 as usize)
    }
}
