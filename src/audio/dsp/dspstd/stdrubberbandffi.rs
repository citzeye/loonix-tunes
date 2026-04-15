/* --- LOONIX-TUNES src/audio/dsp/rubberbandffi.rs --- */
use std::os::raw::{c_double, c_int, c_uint};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StdRubberBandState_t {
    _unused: [u8; 0],
}
pub type StdRubberBandState = *mut StdRubberBandState_t;

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
    ) -> StdRubberBandState;

    pub fn rubberband_delete(state: StdRubberBandState);
    pub fn rubberband_reset(state: StdRubberBandState);
    pub fn rubberband_set_pitch_scale(state: StdRubberBandState, scale: c_double);
    pub fn rubberband_set_time_ratio(state: StdRubberBandState, ratio: c_double);
    pub fn rubberband_process(
        state: StdRubberBandState,
        input: *const *const f32,
        samples: c_uint,
        final_step: c_int,
    );
    pub fn rubberband_available(state: StdRubberBandState) -> c_int;
    pub fn rubberband_retrieve(
        state: StdRubberBandState,
        output: *mut *mut f32,
        samples: c_uint,
    ) -> c_uint;
    pub fn rubberband_get_latency(state: StdRubberBandState) -> c_uint;
}
