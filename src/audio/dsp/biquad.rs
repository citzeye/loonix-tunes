/* --- LOONIX-TUNES src/audio/dsp/biquad.rs --- */

use std::f32::consts::PI;

pub struct BiquadHpf {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadHpf {
    pub fn new() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn update_coefficients(&mut self, sample_rate: f32, cutoff_freq: f32, q_factor: f32) {
        let w0 = 2.0 * PI * cutoff_freq / sample_rate;
        let alpha = w0.sin() / (2.0 * q_factor);
        let cos_w0 = w0.cos();

        let b0_raw = (1.0 + cos_w0) / 2.0;
        let b1_raw = -(1.0 + cos_w0);
        let b2_raw = (1.0 + cos_w0) / 2.0;
        let a0_raw = 1.0 + alpha;
        let a1_raw = -2.0 * cos_w0;
        let a2_raw = 1.0 - alpha;

        self.b0 = b0_raw / a0_raw;
        self.b1 = b1_raw / a0_raw;
        self.b2 = b2_raw / a0_raw;
        self.a1 = a1_raw / a0_raw;
        self.a2 = a2_raw / a0_raw;
    }

    #[inline(always)]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

pub struct BiquadLowShelf {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadLowShelf {
    pub fn new() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn update_coefficients(
        &mut self,
        sample_rate: f32,
        cutoff_freq: f32,
        gain_db: f32,
        q_factor: f32,
    ) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * cutoff_freq / sample_rate;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q_factor);

        let sq_a = a.sqrt();
        let a_plus_1 = a + 1.0;
        let a_minus_1 = a - 1.0;

        let b0_raw = a * (a_plus_1 - a_minus_1 * cos_w0 + 2.0 * sq_a * alpha);
        let b1_raw = 2.0 * a * (a_minus_1 - a_plus_1 * cos_w0);
        let b2_raw = a * (a_plus_1 - a_minus_1 * cos_w0 - 2.0 * sq_a * alpha);
        let a0_raw = a_plus_1 + a_minus_1 * cos_w0 + 2.0 * sq_a * alpha;
        let a1_raw = -2.0 * (a_minus_1 + a_plus_1 * cos_w0);
        let a2_raw = a_plus_1 + a_minus_1 * cos_w0 - 2.0 * sq_a * alpha;

        self.b0 = b0_raw / a0_raw;
        self.b1 = b1_raw / a0_raw;
        self.b2 = b2_raw / a0_raw;
        self.a1 = a1_raw / a0_raw;
        self.a2 = a2_raw / a0_raw;
    }

    #[inline(always)]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }

    pub fn reset(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}
