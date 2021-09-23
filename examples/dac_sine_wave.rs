#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f3xx_hal::{dac::{DacDevice, DacBitAlignment, Trigger, DacChannel}, delay::{self, Delay}, pac, prelude::*, timer::Timer};
    

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

    // Output frequency of the DAC
    let output_frequency = 2000;
    let timer_frequency = output_frequency * x;

    // set up timer 6
    let dac_timer = Timer::new(dp.TIM6, clocks, apb);
    dac_timer.configure_interrupt(event, enable);

    let mut delay = Delay::new(syst, clocks);

    // set up dac1
    let mut dac1 = Dac::new(dp.DAC1, DacDevice::One, DacBitAlignment::TwelveR, 3.3);
    dac1.set_trigger(DacChannel::One, Trigger::Tim6);

    //set up DMA
    let mut dma = Dma



   

    
    

    // Be aware that the values in the table below depend on the input of VREF.
    // To have a stable VREF input, put a condensator and a volt limiting diode in front of it.
    //
    // Also know that integer division and the ADC hardware unit always round down.
    // To make up for those errors, see this forum entry:
    // [https://forum.allaboutcircuits.com/threads/why-adc-1024-is-correct-and-adc-1023-is-just-plain-wrong.80018/]
    hprintln!("
    The ADC has a 12 bit resolution, i.e. if your reference Value is 3V:
        approx. ADC value | approx. volt value
        ==================+===================
                        0 |        0 mV
                     2048 |     1500 mV
                     4095 |     3000 mV

    If you are using a STM32F3Discovery, PA0 is connected to the User Button.
    Pressing it should connect the user Button to to HIGH and the value should change from 0 to 4095.
    ").expect("Error using hprintln.");

    loop {
        let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading adc1.");
        hprintln!("PA0 reads {}", adc1_in1_data).ok();
        asm::delay(2_000_000);
    }
}
