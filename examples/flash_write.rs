#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_semihosting as _;
use stm32f3xx_hal::{pac, prelude::*};

// stolen from another stm32f3 example memory.x, I hope this lines up
const RAM_START: u32 = 0x20000000;
const PAGE_SZE: u32 = 0x7d0; // 2 KB

#[entry]
/// Main Thread
fn main() -> ! {
    let test_address = RAM_START - PAGE_SZE;
    // Get peripherals, clocks and freeze them
    let dp = pac::Peripherals::take().unwrap();

    defmt::error!("erasing flash page at {}...", test_address);
    dp.FLASH.page_erase(test_address).unwrap();

    // make sure function is diverging
    loop {
        asm::nop();
    }
}
