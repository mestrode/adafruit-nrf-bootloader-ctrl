//! Bootloader modules for nrf52 boards
//! Provides functions to reset the decive into different bootloader modes

#![no_std]

pub mod reset;

/// adafruit nrf52 bootloader controller
/// 
/// This module provides functions to reset the device into different bootloader modes.
/// This is useful for example to reset into DFU mode to update the firmware.
/// 
/// Adadrfuit bootloader repo: <https://github.com/adafruit/Adafruit_nRF52_Bootloader>
/// 
/// Bootloader Update: <https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather/update-bootloader>

pub use reset::*;
