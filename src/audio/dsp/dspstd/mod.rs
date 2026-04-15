/* --- LOONIX-TUNES src/audio/dsp/std/mod.rs --- */

pub mod stdabrepeat;
pub mod stdbassbooster;
pub mod stdcompressor;
pub mod stdcrossfeed;
pub mod stdcrystalizer;
pub mod stdeq;
pub mod stdeqpreamp;
pub mod stdlimiter;
pub mod stdmiddleclarity;
pub mod stdnormalizer;
pub mod stdpitchshifter;
pub mod stdreverb;
pub mod stdstereoenhance;
pub mod stdstereowidth;
pub mod stdsurround;

// preamp

pub use self::stdeqpreamp::{get_preamp_enabled_arc, get_preamp_gain_arc, StdEqPreamp};
//limiter
pub use self::stdlimiter::{get_limiter_enabled_arc, StdLimiter};

// 2. Re-export Struct & Getters (Pola identik dengan pro/mod.rs menggunakan prefix Std)

pub use self::stdabrepeat::StdABRepeat;

pub use self::stdbassbooster::{
    get_bass_enabled_arc, get_bass_freq_arc, get_bass_gain_arc, get_bass_q_arc,
    BassBooster as StdBassBooster,
};

pub use self::stdcompressor::{
    get_compressor_enabled_arc, get_compressor_threshold_arc, StdCompressor,
};

pub use self::stdcrossfeed::{get_crossfeed_amount_arc, get_crossfeed_enabled_arc, StdCrossfeed};

pub use self::stdcrystalizer::{
    get_crystal_amount_arc, get_crystal_enabled_arc, get_crystal_freq_arc, StdCrystalizer,
};

pub use self::stdeq::{get_eq_band_arc, get_eq_bands_arc, get_eq_enabled_arc};

pub use self::stdeq::StdEqProcessor;

pub use self::stdmiddleclarity::{get_middle_amount_arc, get_middle_enabled_arc, StdMiddleClarity};

pub use self::stdnormalizer::{
    get_normalizer_gain_arc, get_normalizer_smoothing_arc, StdAudioNormalizer,
};

pub use self::stdpitchshifter::{get_pitch_enabled_arc, get_pitch_ratio_arc, StdPitchShifter};

pub use self::stdreverb::{get_reverb_preset_arc, StdReverb};

pub use self::stdstereoenhance::{get_stereo_amount_arc, get_stereo_enabled_arc, StdStereoEnhance};

pub use self::stdstereowidth::{get_mono_enabled_arc, get_mono_width_arc, StdStereoWidth};

pub use self::stdsurround::{
    get_surround_enabled_arc, get_surround_width_arc, StdSurroundProcessor,
};
