/* --- LOONIX-TUNES src/audio/pulsebt.rs --- */
//! Background PulseAudio Monitor - Non-Blocking Architecture

#![allow(non_snake_case)]

use libpulse_binding::context::{Context, FlagSet, State};
use libpulse_binding::mainloop::threaded::Mainloop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct SystemAudioStatus {
    pub isMuted: bool,
    pub isBluetooth: bool,
}

pub struct PulseMonitor {
    status: Arc<Mutex<SystemAudioStatus>>,
    running: Arc<AtomicBool>,
}

impl PulseMonitor {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(SystemAudioStatus {
                isMuted: false,
                isBluetooth: false,
            })),
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn startMonitor(&self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let status = Arc::clone(&self.status);
        let running = Arc::clone(&self.running);

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let result = pollPulseAudio();

                if let Ok(mut guard) = status.lock() {
                    guard.isMuted = result.isMuted;
                    guard.isBluetooth = result.isBluetooth;
                }

                thread::sleep(Duration::from_secs(2));
            }
        });
    }

    pub fn getLatestStatus(&self) -> SystemAudioStatus {
        match self.status.try_lock() {
            Ok(guard) => *guard,
            Err(_) => SystemAudioStatus {
                isMuted: false,
                isBluetooth: false,
            },
        }
    }
}

fn pollPulseAudio() -> SystemAudioStatus {
    let default = SystemAudioStatus {
        isMuted: false,
        isBluetooth: false,
    };

    let mut mainloop = match Mainloop::new() {
        Some(m) => m,
        None => return default,
    };

    let mut context = match Context::new(&mainloop, "LoonixMonitor") {
        Some(c) => c,
        None => return default,
    };

    if context.connect(None, FlagSet::NOAUTOSPAWN, None).is_err() {
        return default;
    }

    if mainloop.start().is_err() {
        return default;
    }

    mainloop.lock();

    let mut isReady = false;
    for _ in 0..10 {
        match context.get_state() {
            State::Ready => {
                isReady = true;
                break;
            }
            State::Failed | State::Terminated => break,
            _ => {
                mainloop.wait();
            }
        }
    }

    if !isReady {
        mainloop.unlock();
        mainloop.stop();
        return default;
    }

    let statusArc = Arc::new(Mutex::new(default));
    let statusClone = Arc::clone(&statusArc);

    let mut introspector = context.introspect();
    introspector.get_sink_info_by_name("@DEFAULT_SINK@", move |listResult| {
        if let libpulse_binding::callbacks::ListResult::Item(info) = listResult {
            if let Ok(mut status) = statusClone.lock() {
                status.isMuted = info.mute;
                if let Some(busName) = info.proplist.get_str("device.bus") {
                    status.isBluetooth = busName.to_lowercase().contains("bluetooth");
                }
            }
        }
    });

    mainloop.unlock();
    thread::sleep(Duration::from_millis(20));
    mainloop.stop();

    let result = *statusArc.lock().unwrap();
    result
}

// Global monitor
static MONITOR: std::sync::LazyLock<PulseMonitor, fn() -> PulseMonitor> =
    std::sync::LazyLock::new(PulseMonitor::new);

pub fn getSystemAudioStatus() -> SystemAudioStatus {
    MONITOR.getLatestStatus()
}

pub fn startSystemMonitor() {
    MONITOR.startMonitor();
}
