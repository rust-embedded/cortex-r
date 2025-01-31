//! Code for the *Vector Base Address Register*

/// The *Vector Base Address Register* (VBAR)
///
/// There is no `modify` method because this register holds a single 32-bit address.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Vbar(pub *mut u32);

impl Vbar {
    /// Reads the *Vector Base Address Register*
    #[inline]
    pub fn read() -> Vbar {
        let r: usize;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c12, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r as *mut u32)
    }

    /// Write to the *Vector Base Address Register*
    ///
    /// # Safety
    ///
    /// You must supply a correctly-aligned address of a valid Arm Cortex-R
    /// Vector Table.
    #[inline]
    pub unsafe fn write(_value: Self) {
        // Safety: Writing this register is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 0, {}, c12, c0, 0", in(reg) _value.0, options(nomem, nostack, preserves_flags));
        };
    }
}

impl core::fmt::Debug for Vbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VBAR {{ {:010p} }}", self.0)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Vbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VBAR {{ 0x{=usize:08x} }}", self.0 as usize)
    }
}
