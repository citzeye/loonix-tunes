/* --- LOONIX-TUNES src/audio/dsp/mod.rs --- */

use std::sync::atomic::AtomicBool;
use std::sync::OnceLock;

// SHARED ATOMICS - used by both fx/ and pro/ folders
static PRO_UNLOCKED: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_pro_unlocked_arc() -> &'static AtomicBool {
    PRO_UNLOCKED.get_or_init(|| AtomicBool::new(false))
}

pub mod abrepeat;
pub mod chain;
pub mod eq;
pub mod fx;
pub mod limiter;
pub mod magic;
pub mod normalizer;
pub mod pro;
pub mod rack;

// Only re-export core items - let submodules handle their own
pub use self::abrepeat::{ABRepeat, ABRepeatState};
pub use self::chain::DspChain;
pub use self::limiter::Limiter;
pub use self::normalizer::AudioNormalizer;
pub use self::rack::DspRack;

use crate::audio::engine::ProAudioEngine;

pub trait DspProcessor {
    fn process(&mut self, input: &[f32], output: &mut [f32]);
    fn reset(&mut self);
    fn as_any(&mut self) -> &mut dyn std::any::Any;
    fn as_any_ref(&self) -> &dyn std::any::Any;
}

#[derive(Clone)]
pub struct DspSettings {
    pub preamp_db: f32,
    pub bass_enabled: bool,
    pub bass_gain: f32,
    pub bass_cutoff: f32,
    pub bass_q: f32,
    pub crystal_enabled: bool,
    pub crystal_amount: f32,
    pub crystal_freq: f32,
    pub surround_enabled: bool,
    pub surround_width: f32,
    pub surround_room_size: f32,
    pub surround_bass_safe: bool,
    pub mono_enabled: bool,
    pub mono_width: f32,
    pub pitch_enabled: bool,
    pub pitch_semitones: f32,
    pub middle_enabled: bool,
    pub middle_amount: f32,
    pub compressor_enabled: bool,
    pub stereo_enabled: bool,
    pub stereo_amount: f32,
    pub crossfeed_enabled: bool,
    pub crossfeed_amount: f32,
    pub eq_bands: [f32; 10],
    pub eq_dry: f32,
    pub eq_wet: f32,
}

impl Default for DspSettings {
    fn default() -> Self {
        Self {
            preamp_db: 0.0,
            bass_enabled: false,
            bass_gain: 6.0,
            bass_cutoff: 80.0,
            bass_q: 0.7,
            crystal_enabled: false,
            crystal_amount: 0.20,
            crystal_freq: 4000.0,
            surround_enabled: false,
            surround_width: 1.3,
            surround_room_size: 15.0,
            surround_bass_safe: true,
            mono_enabled: false,
            mono_width: 1.0,
            pitch_enabled: false,
            pitch_semitones: 0.0,
            middle_enabled: false,
            middle_amount: 0.0,
            compressor_enabled: false,
            stereo_enabled: false,
            stereo_amount: 0.0,
            crossfeed_enabled: false,
            crossfeed_amount: 0.0,
            eq_bands: [0.0; 10],
            eq_dry: 0.0,
            eq_wet: 100.0,
        }
    }
}

pub struct EqManager;

impl EqManager {
    pub fn set_band_gain(engine: &mut ProAudioEngine, band_index: i32, gain: f32) {
        if band_index >= 0 && band_index < 10 {
            engine.set_eq_band_gain(band_index, gain);
        }
    }

    pub fn reset_eq(engine: &mut ProAudioEngine) {
        for i in 0..10 {
            engine.set_eq_band_gain(i, 0.0);
        }
    }

    pub fn apply_preset(engine: &mut ProAudioEngine, preset_name: &str) {
        match preset_name {
            "BASS" => {
                let gains = [6.0, 5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
                for (i, &g) in gains.iter().enumerate() {
                    engine.set_eq_band_gain(i as i32, g);
                }
            }
            "POP" => {
                let gains = [-1.5, 2.0, 5.0, 5.5, 4.0, 0.0, -2.0, -2.0, -1.5, -1.5];
                for (i, &g) in gains.iter().enumerate() {
                    engine.set_eq_band_gain(i as i32, g);
                }
            }
            _ => Self::reset_eq(engine),
        }
    }
}

pub struct DspManager;

impl DspManager {
    pub fn create_default_rack() -> DspChain {
        DspChain::default()
    }

    pub fn create_custom_rack(
        include_eq: bool,
        include_compressor: bool,
        include_reverb: bool,
        include_surround: bool,
        include_limiter: bool,
        include_bassbooster: bool,
        include_crystalizer: bool,
    ) -> DspRack {
        let mut rack = DspRack::new();

        if include_eq {
            rack.add_processor(Box::new(pro::EqProcessor::new()));
        }
        if include_compressor {
            rack.add_processor(Box::new(pro::Compressor::new()));
        }
        if include_reverb {
            rack.add_processor(Box::new(pro::Reverb::new()));
        }
        if include_surround {
            rack.add_processor(Box::new(pro::SurroundProcessor::new()));
        }
        if include_limiter {
            rack.add_processor(Box::new(Limiter::new()));
        }
        if include_bassbooster {
            rack.add_processor(Box::new(pro::BassBooster::new()));
        }
        if include_crystalizer {
            rack.add_processor(Box::new(pro::Crystalizer::new()));
        }

        rack
    }

    pub fn create_effects_only_rack() -> DspRack {
        Self::create_custom_rack(false, true, true, true, true, false, false)
    }
}

pub struct DspController;

impl DspController {
    pub fn set_reverb_preset(preset_name: &str) -> u32 {
        let preset_id = match preset_name.to_lowercase().as_str() {
            "stage" => 1,
            "hall" => 2,
            "stadium" => 3,
            _ => 0,
        };

        let arc = pro::get_reverb_preset_arc();
        arc.store(preset_id, std::sync::atomic::Ordering::Relaxed);

        preset_id
    }

    pub fn toggle_reverb(enabled: bool, current_preset: u32) -> u32 {
        let preset_id = if enabled {
            if current_preset > 0 {
                current_preset
            } else {
                1
            }
        } else {
            0
        };

        let arc = pro::get_reverb_preset_arc();
        arc.store(preset_id, std::sync::atomic::Ordering::Relaxed);

        preset_id
    }

    pub fn set_compressor_enabled(enabled: bool) {
        let arc = pro::get_compressor_enabled_arc();
        arc.store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_compressor_threshold(val: f64) {
        let threshold_db = -60.0 + (val * 60.0);
        let arc = pro::get_compressor_threshold_arc();
        arc.store(
            (threshold_db as f32).to_bits(),
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    pub fn get_compressor_threshold() -> f64 {
        let arc = pro::get_compressor_threshold_arc();
        let bits = arc.load(std::sync::atomic::Ordering::Relaxed);
        let threshold_db = f32::from_bits(bits);
        ((threshold_db + 60.0) / 60.0) as f64
    }

    pub fn set_pitch_ratio(semitones: f64) {
        let raw = semitones.max(-12.0).min(12.0);
        let ratio = if raw.abs() < 0.5 {
            0.0
        } else {
            2.0_f32.powf((raw as f32) / 12.0)
        };

        let arc = pro::get_pitch_ratio_arc();
        arc.store(ratio.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_eq_band(band_index: i32, gain: f32) {
        if let Some(arc) = eq::get_eq_band_arc(band_index) {
            arc.store(gain.to_bits(), std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn set_crystalizer_amount(amount: f32) {
        pro::get_crystal_amount_arc().store(amount.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }
}
