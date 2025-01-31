//! Code for the *Hyp Vector Base Address Register*

/// The *Hyp Vector Base Address Register* (Hvbar)
///
/// There is no `modify` method because this register holds a single 32-bit address.
///
/// This is only available in EL2.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Hvbar(*mut u32);

impl Hvbar {
    /// Reads the *Hyp Vector Base Address Register*
    ///
    /// Will cause an exception unless you are in EL2.
    #[inline]
    pub fn read() -> Hvbar {
        let r: usize;
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 4, {}, c12, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self(r as *mut u32)
    }

    /// Write to the *Hyp Vector Base Address Register*
    ///
    /// # Safety
    ///
    /// You must supply a correctly-aligned address of a valid Arm Cortex-R
    /// Vector Table.
    #[inline]
    pub fn write(_value: Self) {
        // Safety: Writing this register is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 0, {}, c12, c0, 0", in(reg) _value.0, options(nomem, nostack, preserves_flags));
        };
    }
}

impl core::fmt::Debug for Hvbar {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HVBAR {{ {:010p} }}", self.0)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Hvbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HVBAR {{ 0x{=usize:08x} }}", self.0 as usize)
    }
}
