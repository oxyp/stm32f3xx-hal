#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_semihosting as _;
use stm32f3xx_hal::{flash::FlashError, pac, prelude::*};

const CCM_RAM_START: u32 = 0x10000000;
const PAGE_SZE: u32 = 0x800; // 2 KiB (2048 byte)

#[entry]
/// Main Thread
fn main() -> ! {
    // TODO make sure that this points to a page *start*?

    let test_address = CCM_RAM_START - PAGE_SZE;

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut ok_led = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut err_busy_led = gpioe
        .pe13
        //red
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut err_erase_failed_led = gpioe
        .pe10
        //orange
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut err_unlock_failed_led = gpioe
        .pe8
        //blue
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // defmt::info!("erasing flash page at {:x}...", test_address);
    // // let erase_result = dp.FLASH.page_erase(test_address);
    // defmt::info!("erase result: {}", erase_result);
    
    // // let erase_result: Result<(), _> = Err(FlashError::Busy);
    // // let erase_result: Result<(), _> = Err(FlashError::EraseFailed);
    // // let erase_result: Result<(), _> = Err(FlashError::UnlockFailed);
    // // let erase_result: Result<(), ()> = Ok(());

    // match erase_result {
    //     Ok(_) => {
    //         for i in 0..10 {
    //             ok_led.set_low().unwrap();
    //             cortex_m::asm::delay(16_000_000);
    //             ok_led.set_high().unwrap();
    //             cortex_m::asm::delay(8_000_000);
    //         } 
    //     },
    //     //South
    //     Err(FlashError::Busy) => {
    //         for i in 0..3 {
    //             err_busy_led.set_low().unwrap();
    //             cortex_m::asm::delay(16_000_000);
    //             err_busy_led.set_high().unwrap();
    //             cortex_m::asm::delay(8_000_000);
    //         }
    //     },
    //     // North East
    //     Err(FlashError::EraseFailed) => {
    //         for i in 0..3 {
    //             err_erase_failed_led.set_low().unwrap();
    //             cortex_m::asm::delay(16_000_000);
    //             err_erase_failed_led.set_high().unwrap();
    //             cortex_m::asm::delay(8_000_000);
    //         }
    //     },
    //     // North West
    //     Err(FlashError::UnlockFailed) => {
    //         for i in 0..3 {
    //             err_unlock_failed_led.set_low().unwrap();
    //             cortex_m::asm::delay(16_000_000);
    //             err_unlock_failed_led.set_high().unwrap();
    //             cortex_m::asm::delay(8_000_000);
    //         }
    //     },
    // }

    // TODO try out write when erase works
    //let erase_result = dp.FLASH.page_write(test_address);
    defmt::info!("erasing done");

    defmt::info!("start writing");
    let data: u32 = 0x01;
    let write_result = dp.FLASH.page_write(test_address, data);

    // make sure function is diverging
    loop {
        asm::nop();
    }
}
