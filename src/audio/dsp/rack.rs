/* --- LOONIX-TUNES src/audio/dsp/rack.rs --- */

// Explicit imports - no globs to avoid ambiguity
use crate::audio::dsp::magic::magicbassbooster::{
    get_bass_enabled_arc, get_bass_freq_arc, get_bass_gain_arc, get_bass_q_arc, BassBooster,
};
use crate::audio::dsp::magic::magiccrystalizer::{
    get_crystal_amount_arc, get_crystal_enabled_arc, Crystalizer,
};
use crate::audio::dsp::magic::magicsurround::{
    get_surround_enabled_arc, get_surround_width_arc, SurroundProcessor,
};

use crate::audio::dsp::fx::fxcrossfeed::{
    get_crossfeed_amount_arc, get_crossfeed_enabled_arc, Crossfeed,
};
use crate::audio::dsp::fx::fxmiddleclarity::{
    get_middle_amount_arc, get_middle_enabled_arc, MiddleClarity,
};
use crate::audio::dsp::fx::fxpitchshifter::{
    get_pitch_enabled_arc, get_pitch_ratio_arc, PitchShifter,
};
use crate::audio::dsp::fx::fxstereoenhance::{
    get_stereo_amount_arc, get_stereo_enabled_arc, StereoEnhance,
};
use crate::audio::dsp::fx::fxstereowidth::{get_mono_enabled_arc, get_mono_width_arc, StereoWidth};

use crate::audio::dsp::eq::{
    eqpreamp::{get_preamp_enabled_arc, get_preamp_gain_arc, Preamp},
    EqProcessor,
};

use crate::audio::dsp::fx::fxreverb::Reverb;
use crate::audio::dsp::{DspProcessor, DspSettings, Limiter};
use std::sync::atomic::Ordering;

pub struct DspRack {
    pub processors: Vec<Box<dyn DspProcessor + Send + Sync>>,
}

impl DspRack {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor(&mut self, processor: Box<dyn DspProcessor + Send + Sync>) {
        self.processors.push(processor);
    }

    pub fn build_processors(settings: &DspSettings) -> Vec<Box<dyn DspProcessor + Send + Sync>> {
        let mut processors: Vec<Box<dyn DspProcessor + Send + Sync>> = Vec::new();

        // 0. Preamp
        get_preamp_enabled_arc().store(true, Ordering::Relaxed);
        get_preamp_gain_arc().store(1.0_f32.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(Preamp::new()));

        // 1. EQ
        processors.push(Box::new(EqProcessor::with_bands(settings.eq_bands)));

        // 2. Crystalizer
        let crystal = Crystalizer::new();
        get_crystal_enabled_arc().store(settings.crystal_enabled, Ordering::Relaxed);
        get_crystal_amount_arc().store(settings.crystal_amount.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(crystal));

        // 3. Surround
        let surround = SurroundProcessor::new();
        get_surround_enabled_arc().store(settings.surround_enabled, Ordering::Relaxed);
        get_surround_width_arc().store(settings.surround_width.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(surround));

        // 4. StereoWidth
        let mono = StereoWidth::new();
        get_mono_enabled_arc().store(settings.mono_enabled, Ordering::Relaxed);
        get_mono_width_arc().store(settings.mono_width.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(mono));

        // 5. PitchShifter
        let pitch = PitchShifter::new();
        get_pitch_enabled_arc().store(settings.pitch_enabled, Ordering::Relaxed);
        let ratio = 2.0_f32.powf(settings.pitch_semitones / 12.0);
        get_pitch_ratio_arc().store(ratio.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(pitch));

        // 6. MiddleClarity
        let middle = MiddleClarity::new();
        get_middle_enabled_arc().store(settings.middle_enabled, Ordering::Relaxed);
        get_middle_amount_arc().store(settings.middle_amount.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(middle));

        // 7. StereoEnhance
        let stereo = StereoEnhance::new();
        get_stereo_enabled_arc().store(settings.stereo_enabled, Ordering::Relaxed);
        get_stereo_amount_arc().store(settings.stereo_amount.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(stereo));

        // 8. BassBooster
        let bass = BassBooster::new();
        get_bass_enabled_arc().store(settings.bass_enabled, Ordering::Relaxed);
        get_bass_gain_arc().store(settings.bass_gain.to_bits(), Ordering::Relaxed);
        get_bass_freq_arc().store(settings.bass_cutoff.to_bits(), Ordering::Relaxed);
        get_bass_q_arc().store(settings.bass_q.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(bass));

        // 9. Crossfeed
        let crossfeed = Crossfeed::new();
        get_crossfeed_enabled_arc().store(settings.crossfeed_enabled, Ordering::Relaxed);
        get_crossfeed_amount_arc().store(settings.crossfeed_amount.to_bits(), Ordering::Relaxed);
        processors.push(Box::new(crossfeed));

        // 10. Compressor (disabled for now - no field in DspSettings)
        // let compressor = Compressor::new();
        // processors.push(Box::new(compressor));

        // 11. Reverb
        let reverb = Reverb::new();
        processors.push(Box::new(reverb));

        // 12. Limiter
        let limiter = Limiter::new();
        processors.push(Box::new(limiter));

        processors
    }
}
