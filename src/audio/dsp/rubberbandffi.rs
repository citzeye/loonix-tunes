/* --- LOONIX-TUNES src/audio/dsp/rubberbandffi.rs --- */
use std::os::raw::{c_double, c_int, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RubberBandState_t {
    _unused: [u8; 0],
}
pub type RubberBandState = *mut RubberBandState_t;

pub const RB_OPTION_PROCESS_REALTIME: c_uint = 0x00000001;
pub const RB_OPTION_PITCH_HIGH_QUALITY: c_uint = 0x02000000;
pub const RB_OPTION_FORMANT_PRESERVED: c_uint = 0x01000000;

extern "C" {
    pub fn rubberband_new(
        sampleRate: c_uint,
        channels: c_uint,
        options: c_uint,
        initialTimeRatio: c_double,
        initialPitchScale: c_double,
    ) -> RubberBandState;

    pub fn rubberband_delete(state: RubberBandState);
    pub fn rubberband_reset(state: RubberBandState);
    pub fn rubberband_set_pitch_scale(state: RubberBandState, scale: c_double);
    pub fn rubberband_set_time_ratio(state: RubberBandState, ratio: c_double);
    pub fn rubberband_process(
        state: RubberBandState,
        input: *const *const f32,
        frames: c_int,
        avail: bool,
    ) -> c_int;
    pub fn rubberband_available_frames(state: RubberBandState) -> c_int;
    pub fn rubberband_available(state: RubberBandState) -> c_int;
    pub fn rubberband_retrieve(
        state: RubberBandState,
        output: *const *mut f32,
        frames: c_int,
    ) -> c_int;
    pub fn rubberband_get_latency(state: RubberBandState) -> c_int;
    pub fn rubberband_set_max_process_size(state: RubberBandState, size: c_int) -> c_int;
    pub fn rubberband_set_expected_input_duration(state: RubberBandState, duration: c_int)
        -> c_int;
    pub fn rubberband_set_transient_mode(state: RubberBandState, mode: c_int);
    pub fn rubberband_set_phase_mode(state: RubberBandState, mode: c_int);
    pub fn rubberband_set_formant_mode(state: RubberBandState, mode: c_int);
    pub fn rubberband_set_formant_peak_path(shape: c_uint);
}
