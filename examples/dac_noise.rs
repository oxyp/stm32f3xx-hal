#![no_std]
#![no_main]

//! Example usage for DAC on STM32F303

use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;

use core::time::Duration;
use stm32f3xx_hal::{
    dac::{
        Dac, DacDevice, DacBitAlignment, Trigger, DacChannel
    }, 
    delay::{self, Delay}, 
    pac, 
    prelude::*, 
    time::duration::Milliseconds,
    timer::{
        Timer, 
        
    }, 
};
  



#[entry]
/// Main Thread
fn main() -> ! {
    // Get peripherals, clocks and freeze them
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);

    // Set up pin PA4 as analog pin.
    // This pin is connected to the user button on the stm32f3discovery board.
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);   
    let mut dac1_out1 = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb); 
    let mut ok_led = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    

    // Output frequency of the DAC
    let output_frequency = 1 as u32;
    let timer_frequency = Milliseconds::new(output_frequency);
    

    // set up timer 6
    let mut dac_timer = Timer::new(dp.TIM6, clocks, &mut rcc.apb1);
    dac_timer.start(timer_frequency);
    

    // let mut delay = Delay::new(syst, clocks);

    // set up dac1
    let mut dac1 = Dac::new(dp.DAC1, DacDevice::One, DacBitAlignment::TwelveRight, 3.3);
    dac1.enable_channel(DacChannel::One);
    dac1.set_trigger(DacChannel::One, Trigger::Tim6);

    

    //enable noise generation
    dac1.enable_triangle_gen(DacChannel::One);
    

    loop {
        
        ok_led.set_low().unwrap();
        cortex_m::asm::delay(16_000_000);
        ok_led.set_high().unwrap();
        cortex_m::asm::delay(8_000_000);
    }
}
