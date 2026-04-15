/* --- LOONIX-TUNES src/audio/dsp/mod.rs --- */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

// LICENSE GATEKEEPER
static PRO_UNLOCKED: OnceLock<AtomicBool> = OnceLock::new();

pub fn get_pro_unlocked_arc() -> &'static AtomicBool {
    PRO_UNLOCKED.get_or_init(|| AtomicBool::new(false))
}

pub fn is_pro_active() -> bool {
    get_pro_unlocked_arc().load(Ordering::Relaxed)
}

// Submodules
pub mod chain;
pub mod dspstd;
pub mod rack;

// Shared modules
pub mod biquad;
pub mod limiter;
pub mod preamp;
pub mod rubberbandffi;

pub use self::chain::DspChain;
pub use self::rack::DspRack;

// Core Trait
pub trait DspProcessor: Send + Sync {
    fn process(&mut self, input: &[f32], output: &mut [f32]);
    fn reset(&mut self);
    fn as_any(&mut self) -> &mut dyn::std::any::Any;
    fn as_any_ref(&self) -> &dyn::std::any::Any;
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
            middle_amount: 0.5,
            compressor_enabled: false,
            stereo_enabled: false,
            stereo_amount: 1.0,
            crossfeed_enabled: false,
            crossfeed_amount: 0.5,
            eq_bands: [0.0; 10],
        }
    }
}

pub struct DspManager {
    dsp_rack: DspRack,
}

impl DspManager {
    pub fn new() -> Self {
        Self {
            dsp_rack: DspRack::new(),
        }
    }

    pub fn build_rack(&mut self, _is_pro: bool) {
        let settings = DspSettings::default();
        self.dsp_rack.processors = DspRack::build_processors(&settings);
    }

    pub fn update_settings(&mut self, _settings: &DspSettings) {}

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        self.dsp_rack.process(input, output);
    }
}

impl Default for DspManager {
    fn default() -> Self {
        Self::new()
    }
}
