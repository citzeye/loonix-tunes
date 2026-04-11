/* --- LOONIX-TUNES src/audio/dsp/fx/mod.rs --- */

pub mod fxcompressor;
pub mod fxcrossfeed;
pub mod fxmiddleclarity;
pub mod fxpitchshifter;
pub mod fxreverb;
pub mod fxrubberbandffi;
pub mod fxstereoenhance;
pub mod fxstereowidth;

pub use self::fxcompressor::{
    get_compressor_enabled_arc, get_compressor_threshold_arc, Compressor,
};
pub use self::fxcrossfeed::{get_crossfeed_amount_arc, get_crossfeed_enabled_arc, Crossfeed};
pub use self::fxmiddleclarity::{get_middle_amount_arc, get_middle_enabled_arc, MiddleClarity};
pub use self::fxpitchshifter::{get_pitch_enabled_arc, get_pitch_ratio_arc, PitchShifter};
pub use self::fxreverb::{
    get_reverb_damp_arc, get_reverb_preset_arc, get_reverb_room_size_arc, Reverb,
};
pub use self::fxrubberbandffi::{
    rubberband_available, rubberband_delete, rubberband_get_latency, rubberband_new,
    rubberband_process, rubberband_reset, rubberband_retrieve, rubberband_set_pitch_scale,
    RubberBandState, RubberBandState_t, RB_OPTION_FORMANT_PRESERVED, RB_OPTION_PITCH_HIGH_QUALITY,
    RB_OPTION_PROCESS_REALTIME,
};
pub use self::fxstereoenhance::{get_stereo_amount_arc, get_stereo_enabled_arc, StereoEnhance};
pub use self::fxstereowidth::{get_mono_enabled_arc, get_mono_width_arc, StereoWidth};
