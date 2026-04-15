/* --- LOONIX-TUNES src/audio/dsp/limiter.rs --- */

use crate::audio::dsp::DspProcessor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

// Global atomics for real-time UI control (lock-free, like compressor pattern)
static LIMITER_ENABLED: OnceLock<Arc<AtomicBool>> = OnceLock::new();

pub fn get_limiter_enabled_arc() -> Arc<AtomicBool> {
    LIMITER_ENABLED
        .get_or_init(|| Arc::new(AtomicBool::new(true)))
        .clone()
}

pub struct Limiter {
    threshold_lin: f32,
    envelope: f32,
    attack_coeff: f32,
    release_coeff: f32,
}

impl Limiter {
    pub fn new() -> Self {
        let sample_rate = 48000.0;
        let attack_ms = 2.0;
        let release_ms = 50.0;

        Self {
            threshold_lin: 10.0_f32.powf(-0.5 / 20.0),
            envelope: 0.0,
            attack_coeff: (-1.0_f32 / (attack_ms * 0.001 * sample_rate)).exp(),
            release_coeff: (-1.0_f32 / (release_ms * 0.001 * sample_rate)).exp(),
        }
    }
}

impl Default for Limiter {
    fn default() -> Self {
        Self::new()
    }
}

impl DspProcessor for Limiter {
    #[inline(always)]
    fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let enabled = get_limiter_enabled_arc().load(Ordering::Relaxed);
        if !enabled {
            output.copy_from_slice(input);
            return;
        }

        let safe_len = input.len() - (input.len() % 2);

        for i in (0..safe_len).step_by(2) {
            let l = input[i];
            let r = input[i + 1];

            let peak = l.abs().max(r.abs());

            if peak > self.envelope {
                self.envelope = peak + self.attack_coeff * (self.envelope - peak);
            } else {
                self.envelope = peak + self.release_coeff * (self.envelope - peak);
            }

            let mut gain = 1.0;
            if self.envelope > self.threshold_lin {
                gain = self.threshold_lin / self.envelope;
            }

            let soft_clip_gain = 1.0;
            let l_limited = (l * gain * soft_clip_gain).tanh();
            let r_limited = (r * gain * soft_clip_gain).tanh();
            output[i] = l_limited;
            output[i + 1] = r_limited;
        }
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}
