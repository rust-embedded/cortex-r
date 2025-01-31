//! Registers example for Arm Cortex-R

#![no_std]
#![no_main]

// pull in our start-up code
use cortex_r as _;
use cortex_r_examples as _;

use semihosting::println;

extern "C" {
    static _stack_top: u32;
}

/// The entry-point to the Rust application.
///
/// It is called by the start-up code in `cortex-m-rt`.
#[no_mangle]
pub extern "C" fn kmain() {
    println!("{:?}", cortex_r::register::Midr::read());
    println!("{:?}", cortex_r::register::Cpsr::read());
    #[cfg(arm_architecture = "v8-r")]
    {
        println!("{:?}", cortex_r::register::Cbar::read());
        println!("{:?}", cortex_r::register::Vbar::read());
        // This only works in EL2 and start-up put us in EL1
        // println!("{:?}", cortex_r::register::Hvbar::read());
    }

    println!("_stack_top: {:010p}", core::ptr::addr_of!(_stack_top));

    println!(
        "{:?} before setting C, I and Z",
        cortex_r::register::Sctlr::read()
    );
    cortex_r::register::Sctlr::modify(|w| {
        w.set_c(true);
        w.set_i(true);
        w.set_z(true);
    });
    println!("{:?} after", cortex_r::register::Sctlr::read());

    semihosting::process::exit(0);
}
