#![allow(non_snake_case)]

use std::sync::atomic::{AtomicBool, Ordering};

static SYSTEM_MUTED: AtomicBool = AtomicBool::new(false);
static BLUETOOTH_DETECTED: AtomicBool = AtomicBool::new(false);

#[derive(Default)]
pub struct SystemAudioStatus {
    pub isMuted: bool,
    pub isBluetooth: bool,
}

pub fn getSystemAudioStatus() -> SystemAudioStatus {
    SystemAudioStatus {
        isMuted: SYSTEM_MUTED.load(Ordering::SeqCst),
        isBluetooth: BLUETOOTH_DETECTED.load(Ordering::SeqCst),
    }
}
