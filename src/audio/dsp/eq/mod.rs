/* --- LOONIX-TUNES src/audio/dsp/eq/mod.rs --- */

pub mod eq;
pub mod eqbiquad;
pub mod eqpreamp;

pub use self::eq::{
    get_eq_band_arc, get_eq_bands_arc, get_eq_dry_arc, get_eq_enabled_arc, get_eq_wet_arc,
    BiquadCoeffs, BiquadFilter, EqProcessor,
};
pub use self::eqbiquad::{BiquadHpf, BiquadLowShelf};
pub use self::eqpreamp::{get_preamp_enabled_arc, get_preamp_gain_arc, Preamp};
