// use std::sync::mpsc::channel;

use cortex_m::asm;

use crate::{
    gpio::{self, Analog},
    pac::{
        DAC1,
        dac1,
    },
    rcc::{Clocks, AHB},
};

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]

use crate::pac::DMA1::{self};

use crate::{
    pac::{
        self,
        rcc,
        RCC,
    },
};




pub enum DacMode {

}

pub enum DacDevice {
    One,
    Two,
}

pub enum DacChannel {
    One,
    Two,
}

pub enum DacBitAlignment {
    /// Eight bit precision, right-aligned.
    EightRight,
    /// 12-bit precision, left-aligned.
    TwelveLeft,
    /// 12-bit precision, right-aligned.
    TwelveRight,
}

pub enum Trigger {
    /// Timer 6
    Tim6 = 0b000,
    /// Timers 3 or 8
    Tim3_8 = 0b001,
    /// Timer 7
    Tim7 = 0b010,
    /// Timer 15
    Tim5 = 0b011,
    /// Timer 2
    Tim2 = 0b100,
    /// Timer 4
    Tim4 = 0b101,
    /// Eg, for interrupts
    Exti9 = 0b110,
    /// A software trigger
    Swtrig = 0b111,
}

// #[cfg(any(feature = "gpio-f303", feature = "gpio-f303e"))]
// adc_pins!(DAC1,
//     gpio::PA4<Analog> => 1,
//     gpio::PA5<Analog> => 2,
//     gpio::PA6<Analog> => 3,
// );

// pub struct C1;
// pub struct C2;

// pub trait DacOut<V> {
//     fn set_value(&mut self, val: V);
//     fn get_value(&mut self) -> V;
// }

// pub trait DacPin {
//     fn enable(&mut self);
// }

// pub trait Pins<DAC> {
//     type Output;
//     #[doc(hidden)]
//     fn init() -> Self::Output;
// }

// impl Pins<DAC> for PA4<Analog> {
//     type Output = C1;
//     fn init() -> Self::Output {
//         C1
//     }
// }

// impl Pins<DAC> for PA5<Analog> {
//     type Output = C2;
//     fn init() -> Self::Output {
//         C2
//     }
// }

/// Represents a Digital to Analog Converter (DAC) peripheral.
pub struct Dac {
    pub regs: DAC1,
    device: DacDevice,
    bits: DacBitAlignment,
    vref: f32,
}

// todo: Calculate the VDDA vref, as you do with onboard ADCs!
impl Dac 
// where
//     DAC: pac::DAC1,
{
    /// Initialize a DAC peripheral, including  enabling and resetting
    /// its RCC peripheral clock. `vref` is in volts.
    pub fn new(regs: DAC1, device: DacDevice, bits: DacBitAlignment, vref: f32) -> Self {
        
            let rcc = unsafe { &(*RCC::ptr()) };
            let apb1rstr = &rcc.apb1rstr;
            match device {
                DacDevice::One => {
                    apb1rstr.reset();

                    
                
                 },
                DacDevice::Two => todo!(),
            };           
        

        Self {
            regs,
            device,
            bits,
            vref,
        }
    }

    pub fn enable_channel(&mut self, channel: DacChannel) {
        let cr = &self.regs.cr;

        cr.modify(|_, w| match channel {
            DacChannel::One => w.en1().set_bit(),
            DacChannel::Two => w.en2().set_bit(),
        });
    }

    /// Select and activate a trigger. See f303 Reference manual, section 16.5.4.
    /// Each time a DAC interface detects a rising edge on the selected trigger source (refer to the
    /// table below), the last data stored into the DAC_DHRx register are transferred into the
    /// DAC_DORx register. The DAC_DORx register is updated three dac_pclk cycles after the
    /// trigger occurs.
    pub fn set_trigger(&mut self, channel: DacChannel, trigger: Trigger) {
        let cr = &self.regs.cr;

        match channel {
            DacChannel::One => {
                cr.modify(|_, w| unsafe {
                    w.ten1().set_bit();
                    w.tsel1().bits(trigger as u8)
                });
            }
            DacChannel::Two => {
                cr.modify(|_, w| unsafe {
                    w.ten2().set_bit();
                    w.tsel2().bits(trigger as u8)
                });
            }
        }
    }

    pub fn enable_noise_gen(&mut self, channel: DacChannel) {
        let cr = &self.regs.cr;

        match channel {
            DacChannel::One => {
                cr.modify(|_, w| unsafe {
                    w.wave1().noise()
                });
            }
            DacChannel::Two => {
                cr.modify(|_, w| unsafe {
                    w.wave2().noise()
                });
            }
        }

    }
}
            
                