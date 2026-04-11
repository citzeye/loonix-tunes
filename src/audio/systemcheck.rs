#![allow(non_snake_case)]

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static SYSTEM_MUTED: AtomicBool = AtomicBool::new(false);

pub fn isSystemMuted() -> bool {
    SYSTEM_MUTED.load(Ordering::Relaxed)
}

pub fn startSystemCheck() {
    thread::spawn(|| loop {
        let output = Command::new("pactl")
            .args(["get-sink-mute", "@DEFAULT_SINK@"])
            .output();

        if let Ok(out) = output {
            let status_str = String::from_utf8_lossy(&out.stdout).to_lowercase();
            let muted = status_str.contains("yes");
            SYSTEM_MUTED.store(muted, Ordering::Relaxed);
        }
        thread::sleep(Duration::from_millis(800));
    });
}
