/* --- LOONIX-TUNES src/audio/audiooutput.rs --- */

use crate::audio::dsp::dspstd::stdcrystalizer::get_crystal_amount_arc;
use crate::audio::dsp::{DspChain, DspProcessor};
use crate::audio::engine::OutputMode;
use libpulse_binding as pa;
use libpulse_simple_binding as pa_simple;
use pa::sample::{Format, Spec};
use pa::stream::Direction;
use ringbuf::traits::{Consumer, Observer};
use ringbuf::HeapCons;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use thread_priority::{set_current_thread_priority, ThreadPriority};

// Audio Commands for the background thread (Lock-Free Communication)
// Thread owns: PulseAudio handle, Ring Buffer Consumer, DSP chain, all processing state
pub enum AudioCommand {
    Play {
        handle: pa_simple::Simple,
        consumer: HeapCons<f32>,
        should_stop: Arc<AtomicBool>,
        seek_mode: Arc<AtomicBool>,
        paused: Arc<AtomicBool>,
        flush_requested: Arc<AtomicBool>,
        seek_fade_remaining: Arc<AtomicU32>,
        volume_bits: Arc<AtomicU32>,
        balance_bits: Arc<AtomicU32>,
        mode: Arc<Mutex<OutputMode>>,
        dsp_chain: DspChain,
        dsp_enabled: Arc<AtomicBool>,
        normalizer_enabled: Arc<AtomicBool>,
        normalizer: Arc<Mutex<crate::audio::dsp::dspstd::stdnormalizer::StdAudioNormalizer>>,
        samples_played: Arc<AtomicU64>,
        empty_callback_count: Arc<AtomicU32>,
    },
    Stop,
    Flush,
    Exit,
}

// Helper to convert f32 to u32 bits for atomic storage
fn f32_to_bits(f: f32) -> u32 {
    f.to_bits()
}

fn bits_to_f32(bits: u32) -> f32 {
    f32::from_bits(bits)
}

// Constants for audio processing
const BUFFER_EMPTY_THRESHOLD: u32 = 100;

// Get list of available output devices
pub fn getAvailableDevices() -> Vec<String> {
    // TODO: Implement with libpulse-binding
    vec!["Default".to_string()]
}

pub struct AudioOutput {
    is_running: Arc<AtomicBool>,
    is_started: Arc<AtomicBool>,
    should_stop: Arc<AtomicBool>,
    // FIX #1: Use AtomicU32 for volume/balance (lock-free)
    volume_bits: Arc<AtomicU32>,
    balance_bits: Arc<AtomicU32>,
    // Mode is rarely changed, keep Mutex but avoid locking in callback if possible
    // For now, we keep Mutex but minimize locking
    pub mode_shared: Arc<Mutex<OutputMode>>,
    pub mode: OutputMode,
    // Command channel to background thread (Lock-Free Actor Pattern)
    command_tx: mpsc::Sender<AudioCommand>,
    // FIX: Thread handle for proper cleanup lifecycle
    thread_handle: Option<thread::JoinHandle<()>>,
    // DSP Chain (lock-free via AtomicPtr)
    dsp_chain: DspChain,
    // True bypass switch for DSP (Send/Return)
    pub dsp_enabled: Arc<AtomicBool>,
    // Sample counter for audio clock
    samples_played: Arc<AtomicU64>,
    sample_rate: u32,
    // Ring buffer capacity (known at creation)
    ring_buffer_capacity: usize,
    // Callback starvation counter for end-of-track detection
    empty_callback_count: Arc<AtomicU32>,
    // Flag to reset samples on loop
    loop_reset: Arc<AtomicBool>,
    // Consumer for buffer access (owned by callback, not shared)
    _consumer: Option<HeapCons<f32>>,
    // Clear request flag - set by engine, cleared by audio callback
    clear_request: Arc<AtomicBool>,
    // Fade-in remaining samples after seek complete (25ms @ 48kHz = 2400 samples stereo)
    seek_fade_remaining: Arc<AtomicU32>,
    // Seek mode - unconditional silence when true
    seek_mode: Arc<AtomicBool>,
    // Pause mode - outputs silence but keeps buffer intact for resume
    paused: Arc<AtomicBool>,
    // Flush requested flag - set by engine, cleared by audio thread after draining
    flush_requested: Arc<AtomicBool>,
    // Resume pending counter - frames to wait before unmuting
    resume_frame_counter: Arc<AtomicU32>,
    // Shadow Preset: shared handle to consumer for crossfade on track change
    shared_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    // Old track consumer: holds old track's consumer during 50ms overlap
    old_track_consumer: Arc<Mutex<Option<HeapCons<f32>>>>,
    // Exclusive mode (PipeWire bypass on Linux)
    // Normalizer enabled (EBU R128 loudness normalization)
    normalizer_enabled: Arc<AtomicBool>,
    // Normalizer processor (EBU R128) - wrapped in Arc<Mutex> for thread-safe mutable access
    normalizer: Arc<Mutex<crate::audio::dsp::dspstd::stdnormalizer::StdAudioNormalizer>>,
    // Normalizer buffers (pre-allocated, zero-alloc in callback)
    norm_input: Vec<f32>,
    norm_output: Vec<f32>,
    // Selected audio device (None = use default)
    selected_device_index: Arc<Mutex<Option<usize>>>,
    // Bluetooth detection (PulseAudio-based)
    #[allow(dead_code)]
    is_bluetooth_detected: Arc<AtomicBool>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        // Create command channel for background thread
        let (tx, rx) = mpsc::channel();

        // Spawn the audio background thread and STORE the handle
        let thread_handle = thread::Builder::new()
            .name("pulseaudio".to_string())
            .spawn(move || {
                Self::audio_thread_loop(rx);
            })
            .ok();

        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            is_started: Arc::new(AtomicBool::new(false)),
            should_stop: Arc::new(AtomicBool::new(false)),
            volume_bits: Arc::new(AtomicU32::new(f32_to_bits(1.0))),
            balance_bits: Arc::new(AtomicU32::new(f32_to_bits(0.0))),
            mode_shared: Arc::new(Mutex::new(OutputMode::Stereo)),
            mode: OutputMode::Stereo,
            command_tx: tx,
            thread_handle,
            dsp_chain: DspChain::default(),
            dsp_enabled: Arc::new(AtomicBool::new(true)),
            samples_played: Arc::new(AtomicU64::new(0)),
            sample_rate: 48000,
            ring_buffer_capacity: 0,
            empty_callback_count: Arc::new(AtomicU32::new(0)),
            loop_reset: Arc::new(AtomicBool::new(false)),
            _consumer: None,
            clear_request: Arc::new(AtomicBool::new(false)),
            seek_fade_remaining: Arc::new(AtomicU32::new(0)),
            seek_mode: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
            flush_requested: Arc::new(AtomicBool::new(false)),
            resume_frame_counter: Arc::new(AtomicU32::new(0)),
            shared_consumer: Arc::new(Mutex::new(None)),
            old_track_consumer: Arc::new(Mutex::new(None)),
            normalizer_enabled: Arc::new(AtomicBool::new(false)),
            normalizer: Arc::new(Mutex::new(
                crate::audio::dsp::dspstd::stdnormalizer::StdAudioNormalizer::new(true, -14.0),
            )),
            norm_input: vec![0.0f32; 16384],
            norm_output: vec![0.0f32; 16384],
            selected_device_index: Arc::new(Mutex::new(None)),
            is_bluetooth_detected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request_loop_reset(&self) {
        self.loop_reset.store(true, Ordering::SeqCst);
    }

    pub fn get_dsp_chain(&self) -> DspChain {
        self.dsp_chain.clone()
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        if !self.is_started.load(Ordering::SeqCst) {
            self.sample_rate = sample_rate;
        }
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_samples_played_arc(&self) -> Arc<AtomicU64> {
        self.samples_played.clone()
    }

    pub fn get_samples_played(&self) -> u64 {
        self.samples_played.load(Ordering::SeqCst)
    }

    pub fn set_samples_played(&self, samples: u64) {
        self.samples_played.store(samples, Ordering::SeqCst);
    }

    pub fn reset_samples_played(&self, samples: u64) {
        self.samples_played.store(samples, Ordering::SeqCst);
    }

    // Clear ring buffer - use atomic flag (synchronized, NOT via channel)
    // This works even when push_loop_owned() is running
    pub fn clear_buffer(&self) {
        // Set flush flag - push_loop_owned() will check this in its loop
        self.flush_requested.store(true, Ordering::SeqCst);
    }

    pub fn is_buffer_empty(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return c.is_empty();
            }
        }
        true
    }

    // Legacy alias
    pub fn is_ring_buffer_empty(&self) -> bool {
        self.is_buffer_empty()
    }

    pub fn is_truly_buffer_empty(&self) -> bool {
        self.empty_callback_count.load(Ordering::Relaxed) >= BUFFER_EMPTY_THRESHOLD
    }

    // Get ring buffer length (samples available to play)
    pub fn get_buffer_len(&self) -> usize {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                // Use known capacity + is_empty check
                // If not empty, assume full capacity (conservative estimate)
                // This is accurate enough for the hysteresis check
                if !c.is_empty() {
                    return self.ring_buffer_capacity;
                }
                return 0;
            }
        }
        0
    }

    // Check if ring buffer has data (for FIX 5 - buffer guarantee)
    pub fn is_ring_buffer_ready(&self) -> bool {
        if let Ok(cons) = self.shared_consumer.lock() {
            if let Some(ref c) = *cons {
                return !c.is_empty();
            }
        }
        false
    }

    // Check if in seek mode
    pub fn is_seek_mode(&self) -> bool {
        self.seek_mode.load(Ordering::SeqCst)
    }

    // Reset DSP chain (clear effect tails)
    pub fn reset_dsp(&self) {
        self.dsp_chain.reset();
    }

    // Update DSP chain with new settings
    pub fn update_dsp(&mut self, _settings: &crate::audio::dsp::DspSettings) {
        let rack = crate::audio::dsp::DspRack::build_rack(false);
        self.dsp_chain.swap_chain(rack);
    }

    // Fungsi sinkronisasi saat mode berubah di core.rs
    pub fn update_mode_internal(&self) {
        // Only lock when actually changing mode (rare)
        if let Ok(mut m) = self.mode_shared.lock() {
            *m = self.mode;
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume_bits
            .store(f32_to_bits(volume), Ordering::SeqCst);
    }

    pub fn set_balance(&self, balance: f32) {
        self.balance_bits
            .store(f32_to_bits(balance), Ordering::SeqCst);
    }

    pub fn set_dsp_enabled(&self, enabled: bool) {
        self.dsp_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn is_dsp_enabled(&self) -> bool {
        self.dsp_enabled.load(Ordering::SeqCst)
    }

    pub fn set_normalizer_enabled(&mut self, enabled: bool) {
        self.normalizer_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn set_normalizer_gain(&self, gain: f32) {
        if let Ok(mut norm) = self.normalizer.lock() {
            norm.set_fixed_gain(gain);
        }
    }

    pub fn get_normalizer_arc(
        &self,
    ) -> Arc<Mutex<crate::audio::dsp::dspstd::stdnormalizer::StdAudioNormalizer>> {
        self.normalizer.clone()
    }

    #[allow(deprecated)]
    pub fn get_output_devices(&self) -> Vec<String> {
        // TODO: Implement with libpulse-binding
        vec!["Default".to_string()]
    }

    pub fn set_output_device(&self, index: usize) {
        if let Ok(mut selected) = self.selected_device_index.lock() {
            *selected = Some(index);
        }
    }

    pub fn get_selected_device_index(&self) -> Option<usize> {
        self.selected_device_index
            .lock()
            .ok()
            .and_then(|guard| *guard)
    }

    pub fn select_device(&mut self, device_name: String) {
        // TODO: Implement with libpulse-binding device enumeration
        let _ = device_name;
    }

    pub fn selectDevice(&mut self, _deviceName: String) {
        // TODO: Implement with libpulse-binding
    }

    pub fn start(&mut self, consumer: HeapCons<f32>, clear_old: bool, buffer_capacity: usize) {
        // Clear old track consumer if this is a fresh start
        if clear_old {
            if let Ok(mut xf) = self.old_track_consumer.lock() {
                *xf = None;
            }
            self.seek_fade_remaining.store(0, Ordering::SeqCst);
        }

        self.ring_buffer_capacity = buffer_capacity;

        // Create PulseAudio Simple stream with FLOAT32 format
        let spec = Spec {
            format: Format::F32le,
            channels: 2,
            rate: self.sample_rate,
        };

        match pa_simple::Simple::new(
            None,
            "loonix-tunes",
            Direction::Playback,
            None,
            "playback",
            &spec,
            None,
            None,
        ) {
            Ok(handle) => {
                // Use the should_stop flag from the struct, not a local one
                self.should_stop.store(false, Ordering::SeqCst);
                self.is_running.store(true, Ordering::SeqCst);

                let _ = self.command_tx.send(AudioCommand::Play {
                    handle,
                    consumer,
                    should_stop: self.should_stop.clone(),
                    seek_mode: self.seek_mode.clone(),
                    paused: self.paused.clone(),
                    flush_requested: self.flush_requested.clone(),
                    seek_fade_remaining: self.seek_fade_remaining.clone(),
                    volume_bits: self.volume_bits.clone(),
                    balance_bits: self.balance_bits.clone(),
                    mode: self.mode_shared.clone(),
                    dsp_chain: self.dsp_chain.clone(),
                    dsp_enabled: self.dsp_enabled.clone(),
                    normalizer_enabled: self.normalizer_enabled.clone(),
                    normalizer: self.normalizer.clone(),
                    samples_played: self.samples_played.clone(),
                    empty_callback_count: self.empty_callback_count.clone(),
                });

                self.is_started.store(true, Ordering::SeqCst);
            }
            Err(_) => {
                // Failed to create PulseAudio stream
            }
        }
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        self.should_stop.store(true, Ordering::SeqCst);

        // Reset state for clean transition
        self.seek_mode.store(false, Ordering::SeqCst);
        self.seek_fade_remaining.store(0, Ordering::SeqCst);
        self.resume_frame_counter.store(0, Ordering::SeqCst);

        // Reset filter state to prevent pop on next track
        self.reset_dsp();
    }

    pub fn start_consumers(&self) {
        // PHASE 3: Resume - restart audio processing
        self.is_running.store(true, Ordering::SeqCst);
    }

    pub fn pause(&mut self) {
        self.paused.store(true, Ordering::SeqCst);
    }

    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }

    /// Trigger seek fade-in - 25ms smooth ramp at seek completion
    pub fn trigger_seek_fade(&self) {
        // 25ms fade IN at current sample rate (~2400 samples stereo)
        let fade_samples = (self.sample_rate as f32 * 0.025) as u32;
        self.seek_fade_remaining
            .store(fade_samples, Ordering::SeqCst);
    }

    pub fn set_seek_mode(&self, seeking: bool) {
        self.seek_mode.store(seeking, Ordering::SeqCst);
    }

    /// Trigger resume with delay - waits a few frames before unmuting
    pub fn trigger_delayed_resume(&self) {
        // Wait ~2 frames (~84ms at 24fps) before unmuting
        self.resume_frame_counter.store(2, Ordering::SeqCst);
    }

    /// Called from audio callback to check if we should unmute
    /// Returns true if seek mode should be disabled
    pub fn check_resume_counter(&self) -> bool {
        let remaining = self.resume_frame_counter.load(Ordering::SeqCst);
        if remaining > 0 {
            self.resume_frame_counter
                .store(remaining - 1, Ordering::SeqCst);
            return false;
        }
        true
    }

    pub fn resume(&mut self) {
        self.paused.store(false, Ordering::SeqCst);
        self.is_running.store(true, Ordering::SeqCst);
    }

    // Background Thread Loop - Channel-based State Machine
    // This thread owns: PulseAudio handle, Ring Buffer Consumer, DSP chain
    // Loop runs forever until Exit command - doesn't depend on is_running
    fn audio_thread_loop(rx: mpsc::Receiver<AudioCommand>) {
        // Try to promote thread priority for real-time audio
        let _ = set_current_thread_priority(ThreadPriority::Max);

        let mut current_handle: Option<pa_simple::Simple> = None;
        let mut current_flush: Option<Arc<AtomicBool>> = None;

        loop {
            // Wait for next command (blocking receive)
            match rx.recv() {
                Ok(AudioCommand::Exit) => {
                    break;
                }
                Ok(AudioCommand::Play {
                    handle,
                    consumer,
                    should_stop,
                    seek_mode,
                    paused,
                    flush_requested: flush_req,
                    seek_fade_remaining,
                    volume_bits,
                    balance_bits,
                    mode,
                    dsp_chain,
                    dsp_enabled,
                    normalizer_enabled,
                    normalizer,
                    samples_played,
                    empty_callback_count,
                }) => {
                    current_handle = Some(handle);
                    current_flush = Some(flush_req);

                    // Start the push loop
                    if let (Some(h), Some(flush_flag)) =
                        (current_handle.take(), current_flush.take())
                    {
                        Self::push_loop_owned(
                            h,
                            consumer,
                            should_stop,
                            seek_mode,
                            paused,
                            flush_flag,
                            seek_fade_remaining,
                            volume_bits,
                            balance_bits,
                            mode,
                            dsp_chain,
                            dsp_enabled,
                            normalizer_enabled,
                            normalizer,
                            samples_played,
                            empty_callback_count,
                        );
                    }

                    current_handle = None;
                    current_flush = None;
                }
                Ok(AudioCommand::Stop) => {
                    // Set should_stop to interrupt push_loop immediately
                    // Note: is_running is handled in push_loop
                }
                Ok(AudioCommand::Flush) => {
                    // Flush handle if exists
                    if let Some(ref handle) = current_handle {
                        let _ = handle.flush();
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    // Pure Push Loop - Runs until consumer empty (decoder done) or stopped
    // SINGLE AUTHORITY: Only seek_mode and paused determine silence vs audio
    #[allow(clippy::too_many_arguments)]
    fn push_loop_owned(
        mut handle: pa_simple::Simple,
        mut consumer: HeapCons<f32>,
        should_stop: Arc<AtomicBool>,
        seek_mode: Arc<AtomicBool>,
        paused: Arc<AtomicBool>,
        flush_flag: Arc<AtomicBool>,
        seek_fade_remaining: Arc<AtomicU32>,
        volume_bits: Arc<AtomicU32>,
        balance_bits: Arc<AtomicU32>,
        mode: Arc<Mutex<OutputMode>>,
        dsp_chain: DspChain,
        dsp_enabled: Arc<AtomicBool>,
        normalizer_enabled: Arc<AtomicBool>,
        normalizer: Arc<Mutex<crate::audio::dsp::dspstd::stdnormalizer::StdAudioNormalizer>>,
        samples_played: Arc<AtomicU64>,
        empty_callback_count: Arc<AtomicU32>,
    ) {
        let channels = 2;
        let frames_per_write = 1024usize;
        let samples_per_write = frames_per_write * channels;

        // Pre-allocated buffers for DSP processing
        let mut read_buffer = vec![0.0f32; samples_per_write];
        let mut processed_buffer = vec![0.0f32; samples_per_write];
        let mut norm_input = vec![0.0f32; samples_per_write];
        let mut norm_output = vec![0.0f32; samples_per_write];

        let mut empty_count = 0u32;
        const FLUSH_MAX_ITERATIONS: u32 = 1000;

        // Cache mode once before loop (avoid hot-path mutex lock)
        let current_mode = *mode.lock().unwrap_or_else(|e| e.into_inner());

        loop {
            if should_stop.load(Ordering::SeqCst) {
                break;
            }

            // FLUSH HANDLING: Draining old buffer data before seek prebuffer fills it
            if flush_flag.load(Ordering::SeqCst) {
                // Drain entire consumer buffer
                loop {
                    let drained = consumer.pop_slice(&mut read_buffer);
                    if drained == 0 {
                        break;
                    }
                }
                // Reset empty counter since we just cleared
                empty_count = 0;
                empty_callback_count.store(0, Ordering::Relaxed);
                // Clear flush flag - buffer is now clean
                flush_flag.store(false, Ordering::SeqCst);
                // Reset sample counter so it starts fresh from exact position
                // (will be set by engine via reset_samples_played)
                // Continue to next iteration to check seek_mode
            }

            // SINGLE GATE: Only seek_mode determines silence
            // Engine state is managed externally - audio thread only obeys seek_mode
            let is_seeking = seek_mode.load(Ordering::SeqCst);

            if is_seeking {
                // During seek: Just output silence. Do NOT drain consumer so decoder can pre-fill.
                read_buffer.fill(0.0);
                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                let _ = handle.write(silence);
                // Give decoder a small window to fill the buffer
                std::thread::sleep(std::time::Duration::from_millis(2));
                continue;
            }

            // PAUSE: Output silence but keep buffer intact for resume
            // Unlike seek, we don't sleep here - just output silence at normal pace
            if paused.load(Ordering::SeqCst) {
                read_buffer.fill(0.0);
                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                let _ = handle.write(silence);
                continue;
            }

            // 1. Read from ring buffer (only when NOT seeking and NOT paused)
            read_buffer.fill(0.0);
            let samples_read = consumer.pop_slice(&mut read_buffer);

            if samples_read == 0 {
                empty_callback_count.fetch_add(1, Ordering::Relaxed);
                empty_count += 1;

                // Exit if decoder is done
                if empty_count > BUFFER_EMPTY_THRESHOLD {
                    break;
                }

                let silence = unsafe {
                    std::slice::from_raw_parts(
                        read_buffer.as_ptr() as *const u8,
                        samples_per_write * 4,
                    )
                };
                let _ = handle.write(silence);
                continue;
            }

            empty_callback_count.store(0, Ordering::Relaxed);

            // 2. Update sample counter
            samples_played.fetch_add(samples_read as u64, Ordering::SeqCst);

            // 3. Apply DSP Chain if enabled
            let process_len = samples_read.min(read_buffer.len());
            if dsp_enabled.load(Ordering::SeqCst) {
                dsp_chain.process(
                    &read_buffer[..process_len],
                    &mut processed_buffer[..process_len],
                );
            } else {
                processed_buffer[..process_len].copy_from_slice(&read_buffer[..process_len]);
            }

            // 4. Apply Normalizer if enabled
            if normalizer_enabled.load(Ordering::SeqCst) {
                norm_input[..process_len].copy_from_slice(&processed_buffer[..process_len]);
                if let Ok(mut norm) = normalizer.lock() {
                    norm.process(&norm_input[..process_len], &mut norm_output[..process_len]);
                    processed_buffer[..process_len].copy_from_slice(&norm_output[..process_len]);
                }
            }

            // 5. Apply Volume & Balance
            let vol = f32::from_bits(volume_bits.load(Ordering::Relaxed));
            let bal = f32::from_bits(balance_bits.load(Ordering::Relaxed));
            let left_gain = if bal > 0.0 { 1.0 - bal } else { 1.0 };
            let right_gain = if bal < 0.0 { 1.0 + bal } else { 1.0 };

            // 6. Apply Output Mode, Volume, and Seek Fade-in
            let num_frames = process_len / 2;

            // Check fade-in remaining
            let fade_samples = seek_fade_remaining.load(Ordering::Acquire);

            for frame in 0..num_frames {
                let mut left = processed_buffer[frame * 2];
                let mut right = processed_buffer[frame * 2 + 1];

                // Apply balance
                left *= left_gain;
                right *= right_gain;

                // Apply mode (Mono/Surround/Stereo)
                match current_mode {
                    OutputMode::Mono => {
                        let mono = (left + right) * 0.5;
                        left = mono;
                        right = mono;
                    }
                    OutputMode::Surround => {
                        let diff = (left - right) * 0.3;
                        left += diff;
                        right -= diff;
                    }
                    OutputMode::Stereo => {}
                }

                // Apply volume
                left *= vol;
                right *= vol;

                // Apply seek fade-in (25ms smooth ramp)
                if fade_samples > 0 {
                    // Calculate how many frames to fade in this batch
                    let frames_remaining = (fade_samples / 2) as usize;
                    let fade_this_frame = frame.min(frames_remaining.saturating_sub(1));
                    let fade_gain = if frames_remaining > 0 {
                        (fade_this_frame as f32 + 1.0) / (frames_remaining as f32 + 1.0)
                    } else {
                        1.0
                    };
                    // Use sqrt for perceptual smoothness (natural volume curve)
                    let fade_factor = fade_gain.sqrt();
                    left *= fade_factor;
                    right *= fade_factor;
                }

                // Safety clamp
                if !left.is_finite() {
                    left = 0.0;
                }
                if !right.is_finite() {
                    right = 0.0;
                }
                left = left.clamp(-0.99, 0.99);
                right = right.clamp(-0.99, 0.99);

                processed_buffer[frame * 2] = left;
                processed_buffer[frame * 2 + 1] = right;
            }

            // Decrement fade counter after applying fade to all frames
            if fade_samples > 0 {
                let frames_used = (num_frames as u32).min(fade_samples / 2);
                if frames_used > 0 {
                    seek_fade_remaining.fetch_sub(frames_used * 2, Ordering::SeqCst);
                }
            }

            // 7. Write to PulseAudio (blocking - auto-paces with hardware)
            let bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(processed_buffer.as_ptr() as *const u8, process_len * 4)
            };

            if let Err(_e) = handle.write(bytes) {
                // Write error
            }
        }
    }
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        // Stop processing first
        self.is_running.store(false, Ordering::SeqCst);
        self.is_started.store(false, Ordering::SeqCst);

        // FIX: Send Exit command and wait for thread to finish
        let _ = self.command_tx.send(AudioCommand::Exit);

        // Join thread to ensure clean shutdown
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
