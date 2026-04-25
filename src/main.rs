#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    const SYST_CSR: *mut u32 = 0xE000E010 as *mut u32;
    const SYST_CVR: *mut u32 = 0xE000E018 as *mut u32;
    unsafe {
        const SYST_RVR: *mut u32 = 0xE000E014 as *mut u32;
        // Set load value
        write_volatile(SYST_RVR, 5000000 - 1);
        // Clear current value
        write_volatile(SYST_CVR, 0);
        // Choose CLKSOURCE and enable counter
        write_volatile(SYST_CSR, 1 << 2 | 1 << 0);
    }

    unsafe {
        const RCC_APB2ENR: *mut u32 = 0x4_002_1018 as *mut u32;
        // Enable GPIOC clock
        write_volatile(RCC_APB2ENR, 1 << 4);
        const GPIOC_CRH: *mut u32 = 0x4001_1004 as *mut u32;
        // Configure PC13
        write_volatile(GPIOC_CRH, 1 << 20);
    }

    loop {
        const GPIOC_BSRR: *mut u32 = 0x4001_1010 as *mut u32;
        unsafe {
            // Set PC13
            write_volatile(GPIOC_BSRR, 1 << 13);
        }
        unsafe {
            // Clear COUNTFLAG
            write_volatile(SYST_CVR, 0);
        }
        while unsafe { (read_volatile(SYST_CSR) & (1 << 16)) == 0 } {}

        unsafe {
            // Reset PC13
            write_volatile(GPIOC_BSRR, 1 << 29);
        }
        unsafe {
            // Clear COUNTFLAG
            write_volatile(SYST_CVR, 0);
        }
        while unsafe { (read_volatile(SYST_CSR) & (1 << 16)) == 0 } {}
    }
}
