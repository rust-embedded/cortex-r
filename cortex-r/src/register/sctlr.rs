//! Code for managing the *System Control Register*

/// The *System Control Register* (SCTLR)
#[bitbybit::bitfield(u32)]
pub struct Sctlr {
    /// The bitmask for the Instruction Endianness bit
    #[bits(31..=31, rw)]
    ie: bool,
    /// The bitmask for the Thumb Exception Enable bit
    #[bits(30..=30, rw)]
    te: bool,
    /// The bitmask for the Non-Maskable FIQ bit
    #[bits(27..=27, rw)]
    nmfi: bool,
    /// The bitmask for the Exception Endianness bit
    #[bits(25..=25, rw)]
    ee: bool,
    /// The bitmask for the U bit
    #[bits(22..=22, rw)]
    u: bool,
    /// The bitmask for the Fast Interrupt bit
    #[bits(21..=21, rw)]
    fi: bool,
    /// The bitmask for the Divide by Zero Fault bit
    #[bits(18..=18, rw)]
    dz: bool,
    /// The bitmask for the Background Region bit
    #[bits(17..=17, rw)]
    br: bool,
    /// The bitmask for the Round Robin bit
    #[bits(14..=14, rw)]
    rr: bool,
    /// The bitmask for the Exception Vector Table bit
    #[bits(13..=13, rw)]
    v: bool,
    /// The bitmask for the Instruction Cache enable bit
    #[bits(12..=12, rw)]
    i: bool,
    /// The bitmask for the Branch Prediction enable bit
    #[bits(11..=11, rw)]
    z: bool,
    /// The bitmask for the SWP bit
    #[bits(10..=10, rw)]
    sw: bool,
    /// The bitmask for the Cache enable bit
    #[bits(2..=2, rw)]
    c: bool,
    /// The bitmask for the Alignment check bit
    #[bits(1..=1, rw)]
    a: bool,
    /// The bitmask for the MPU bit
    #[bits(0..=0, rw)]
    m: bool,
}

impl Sctlr {
    /// Reads the *System Control Register*
    #[inline]
    pub fn read() -> Self {
        let r: u32;
        // Safety: Reading this register has no side-effects and is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mrc p15, 0, {}, c1, c0, 0", out(reg) r, options(nomem, nostack, preserves_flags));
        }
        #[cfg(not(target_arch = "arm"))]
        {
            r = 0;
        }
        Self::new_with_raw_value(r)
    }

    /// Write to the *System Control Register*
    #[inline]
    pub fn write(_value: Self) {
        // Safety: Writing this register is atomic
        #[cfg(target_arch = "arm")]
        unsafe {
            core::arch::asm!("mcr p15, 0, {}, c1, c0, 0", in(reg) _value.raw_value(), options(nomem, nostack, preserves_flags));
        };
    }

    /// Modify the *System Control Register*
    #[inline]
    pub fn modify<F>(f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut value = Self::read();
        f(&mut value);
        Self::write(value);
    }
}

impl core::fmt::Debug for Sctlr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SCTLR {{ IE={} TE={} NMFI={} EE={} U={} FI={} DZ={} BR={} RR={} V={} I={} Z={} SW={} C={} A={} M={} }}",
            self.ie() as u8,
            self.te() as u8,
            self.nmfi() as u8,
            self.ee() as u8,
            self.u() as u8,
            self.fi() as u8,
            self.dz() as u8,
            self.br() as u8,
            self.rr() as u8,
            self.v() as u8,
            self.i() as u8,
            self.z() as u8,
            self.sw() as u8,
            self.c() as u8,
            self.a() as u8,
            self.m() as u8,
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Sctlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCTLR {{ IE={0=31..32} TE={0=30..31} NMFI={0=27..28} EE={0=25..26} U={0=22..23} FI={0=21..22} DZ={0=18..19} BR={0=17..18} RR={0=14..15} V={0=13..14} I={0=12..13} Z={0=11..12} SW={0=10..11} C={0=2..3} A={0=1..2} M={0=0..1} }}", self.0)
    }
}
