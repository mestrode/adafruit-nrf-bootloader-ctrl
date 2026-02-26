//! Bootloader modules for nrf52 boards
//! Provides functions to reset the decive into different bootloader modes

#![no_std]

use nrf52840_hal as hal;

/// Magic numbers for bootloader configuration
/// see https://github.com/adafruit/Adafruit_nRF52_Bootloader/blob/master/src/main.c#L96
enum BootloaderMagic {
    /// Magiv value for jump to OTA application
    OtaAppJum = 0xB1,
    /// Magic value to reset into OTA mode
    OtaReset = 0xA8,
    /// Magic value to reset into serial mode
    SerialOnlyReset = 0x4e,
    /// Magic value to reset into UF2 mode
    Uf2Reset = 0x57,
    /// Magic value to skip bootloader
    SkipBootloader = 0x6d,
}

/// Resets the device into Device Firmware Update mode (DFU).
pub fn reset_into_app() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR)
            .gpregret
            .write(|w| w.bits(BootloaderMagic::OtaAppJum as u32))
    };
    hal::pac::SCB::sys_reset();
}

/// Resets into OTA mode, which will wait for a new firmware to be writte.
pub fn reset_into_ota() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR)
            .gpregret
            .write(|w| w.bits(BootloaderMagic::OtaReset as u32))
    };
    hal::pac::SCB::sys_reset();
}

/// Resets into serial only mode.
pub fn reset_into_serial_only() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR)
            .gpregret
            .write(|w| w.bits(BootloaderMagic::SerialOnlyReset as u32))
    };
    hal::pac::SCB::sys_reset();
}

/// Resets the device into Device Firmware Update mode (UF2).
pub fn reset_into_uf2() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR)
            .gpregret
            .write(|w| w.bits(BootloaderMagic::Uf2Reset as u32))
    };
    hal::pac::SCB::sys_reset();
}

/// Resets and skips the bootloader. This will reset into the application
pub fn reset_skip_bootloader() -> ! {
    unsafe {
        (*hal::pac::POWER::PTR)
            .gpregret
            .write(|w| w.bits(BootloaderMagic::SkipBootloader as u32))
    };
    hal::pac::SCB::sys_reset();
}
