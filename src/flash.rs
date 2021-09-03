//! # Flash memory
//!
//! Abstractions of the internal flash module.

use crate::pac::{flash, FLASH};
use cortex_m::asm;

/*
SCHLACHTPLAN

- implement flash **page** erase
- ??? check that it worked? (how?)
    * maybe there's a flash read at offset x that we can use?
- implement programming
- ??? check that it worked? (how?)

EXTRA BONUS POINTS

- adjust linker script so that there's a dedicated DATA section
- add boundary checks when erasing/writing
*/

const FLASH_KEYR_KEY_1: u32 = 0x45670123;
const FLASH_KEYR_KEY_2: u32 = 0xCDEF89AB;

// TODO impl std::Error for this?
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashError {
    Busy,
    EraseFailed,
}

/// Extension trait to constrain the FLASH peripheral
pub trait FlashExt {
    /// Constrains the FLASH peripheral to play nicely with the other abstractions
    fn constrain(self) -> Parts;
    fn page_write(self) -> Result<(), ()>;
    fn page_erase(self, address: u32) -> Result<(), FlashError>;
}

impl FlashExt for FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR { _0: () },
        }
    }

    /// TODO write docs
    fn page_erase(self, address: u32) -> Result<(), FlashError> {
        // 1. Check that no main Flash memory operation is ongoing by checking the BSY bit in
        //    the FLASH_SR register.
        if self.sr.read().bsy().bit_is_set() {
            // We are busy! Come back later
            return Err(FlashError::Busy);
        }

        // TODO is the order correct here?
        unlock_cr(&self);

        // 2. Set the PER bit in the FLASH_CR register
        self.cr.write(|w| w.per().set_bit());

        // 3. Program the FLASH_AR register to select a page to erase
        self.ar.write(|w| unsafe { w.bits(address) });

        // 4. Set the STRT bit in the FLASH_CR register (see below note)
        self.cr.write(|w| w.strt().set_bit());

        // 5. Wait for the BSY bit to be reset
        while self.sr.read().bsy().bit_is_set() {
            // do nothing while the BSY bit is not reset yet
            asm::nop();
        }

        // 6. Check the EOP flag in the FLASH_SR register (it is set when the erase operation has succeeded),
        //    and then clear it by software.
        if self.sr.read().eop().bit_is_set() {
            // erase was successful
            // 7. Clear the EOP flag.
            self.sr.write(|w| w.eop().clear_bit())
        } else {
            // this should be set by now!
            return Err(FlashError::EraseFailed);
        }

        // The software should start checking if the BSY bit equals ‘0’ at least one CPU cycle after setting the STRT bit.

        // WE ARE ASSUMING that the above takes > cycle so we're not waiting explicitly (danger danger)
        if self.sr.read().bsy().bit_is_set() {
            Ok(())
        } else {
            Err(FlashError::Busy)
        }
    }

    /// TODO write docs!
    // TODO finish implementation
    fn page_write(self) -> Result<(), ()> {
        // TODO: do we have to unlock write protection (see "Unlocking the Flash memory")?

        // 1. Check that no main Flash memory operation is ongoing by checking the BSY bit in
        //    the FLASH_SR register.
        if self.sr.read().bsy().bit_is_set() {
            // We are busy! Come back later
            // TODO proper error tyoe
            return Err(());
        }

        // TODO is the order correct here?
        unlock_cr(&self);

        // 2. Set the PG bit in the FLASH_CR register.
        self.cr.write(|w| w.pg().bit(true));

        // 3. Perform the data write (half-word) at the desired address.

        // 4. Wait until the BSY bit is reset in the FLASH_SR register.
        // 5. Check the EOP flag in the FLASH_SR register (it is set when the programming operation
        //    has succeeded), and then clear it by software.

        Ok(())
    }
}

/// An unlocking sequence should be written to the FLASH_KEYR register to open the access to
/// the FLASH_CR register. This sequence consists of two write operations into FLASH_KEYR register:
/// 1. Write KEY1 = 0x45670123
/// 2. Write KEY2 = 0xCDEF89AB
/// Any wrong sequence locks up the FPEC and the FLASH_CR register until the next reset.
fn unlock_cr(flash: &FLASH) {
    flash.keyr.write(|w| w.fkeyr().bits(FLASH_KEYR_KEY_1));
    flash.keyr.write(|w| w.fkeyr().bits(FLASH_KEYR_KEY_2));
}

/// Constrained FLASH peripheral
pub struct Parts {
    /// Opaque Access Control Register (ACR)
    pub acr: ACR,
}

/// Opaque Access Control Register (ACR)
pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}
