#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_semihosting as _;
use stm32f3xx_hal::{pac, prelude::*};

const CCM_RAM_START: u32 = 0x10000000;
const PAGE_SZE: u32 = 0x7d0; // 2 KB

#[entry]
/// Main Thread
fn main() -> ! {
    // TODO make sure that this points to a page *start*?

    let test_address = 268433000; //CCM_RAM_START - PAGE_SZE;

    let dp = pac::Peripherals::take().unwrap();

    defmt::info!("erasing flash page at {:x}...", test_address);
    let erase_result = dp.FLASH.page_erase(test_address);
    defmt::info!("erase result: {}", erase_result);

    // TODO try out write when erase works
    //let erase_result = dp.FLASH.page_write(test_address);

    // make sure function is diverging
    loop {
        asm::nop();
    }
}
