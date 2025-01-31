//! Code for managing the *Main ID Register*

use arbitrary_int::{u12, u4};

/// The *Main ID Register* (MIDR)
#[bitbybit::bitfield(u32)]
pub struct Midr {
    /// Implementer
    #[bits(24..=31, r)]
    implementer: u8,
    /// Variant
    #[bits(20..=23, r)]
    variant: u4,
    /// Architecture
    #[bits(16..=19, r)]
    arch: u4,
    /// Part Number
    #[bits(4..=15, r)]
    part_no: u12,
    /// Revision
    #[bits(0..=3, r)]
    rev: u4,
}

impl Midr {
    /// Reads the *Main ID Register*
    #[inline]
    pub fn read() -> Midr {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c0, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self::new_with_raw_value(r)
    }
}

impl core::fmt::Debug for Midr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MIDR {{ implementer=0x{:02x} variant=0x{:x} arch=0x{:x} part_no=0x{:03x} rev=0x{:x} }}",
        self.implementer(), self.variant(), self.arch(), self.part_no(), self.rev())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Midr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MIDR {{ implementer=0x{0=24..32:02x} variant=0x{0=20..24:x} arch=0x{0=16..20:x} part_no=0x{0=4..16:03x} rev=0x{0=0..4:x} }}", self.0)
    }
}
