/* --- LOONIX-TUNES src/audio/dsp/magic/mod.rs --- */

pub mod magicbassbooster;
pub mod magiccrystalizer;
pub mod magicsurround;

pub use self::magicbassbooster::{
    get_bass_enabled_arc, get_bass_freq_arc, get_bass_gain_arc, get_bass_magic_mode_arc,
    get_bass_q_arc, BassBooster,
};
pub use self::magiccrystalizer::{
    get_crystal_amount_arc, get_crystal_enabled_arc, get_crystal_freq_arc,
    get_crystal_magic_mode_arc, Crystalizer,
};
pub use self::magicsurround::{
    get_surround_bass_safe_arc, get_surround_enabled_arc, get_surround_magic_mode_arc,
    get_surround_width_arc, SurroundProcessor,
};
