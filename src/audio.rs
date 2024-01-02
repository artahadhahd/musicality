#![allow(dead_code, non_upper_case_globals)]
const ECoInit: u8 = 1;
const EEnumerator: u8 = 2;
const EDevice: u8 = 3;
const ECreateAudioClient: u8 = 4;

extern "C" {
    fn initialize() -> u8;
    fn enable_debug();
    fn platform_supported() -> bool;
}

pub struct AudioDevice;

impl AudioDevice {
    pub fn new() -> Result<Self, &'static str> {
        if unsafe { initialize() } != 0 {
            return Err("Audio device could not be initialized");
        }
        Ok(Self)
    }

    pub fn enable_debug_mode() {
        unsafe { enable_debug() };
    }

    pub fn supports() -> bool {
        unsafe { platform_supported() }
    }
}
