#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m_rt::entry;
use cortex_m::asm::nop;
use panic_halt as _;

#[entry]
fn main() -> ! {
    const RCC_APB2ENR: *mut u32 = 0x4_002_1018 as *mut u32;
    const GPIOC_CRH: *mut u32 = 0x4001_1004 as *mut u32;
    const GPIOC_BSRR: *mut u32 = 0x4001_1010 as *mut u32;

    unsafe {
        write_volatile(RCC_APB2ENR, 1 << 4);
        write_volatile(GPIOC_CRH, 1 << 20);
    }

    loop {
        unsafe {
            write_volatile(GPIOC_BSRR, 1 << 13);
        }
        for _ in 0..10_000 {
            nop();
        }
        unsafe {
            write_volatile(GPIOC_BSRR, 1 << 29);
        }
        for _ in 0..10_000 {
            nop();
        }
    }
}
