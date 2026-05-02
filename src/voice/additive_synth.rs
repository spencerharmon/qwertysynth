use crate::wave_table::WaveTable;
use crate::voice::Voice;

// Number of harmonic partials. Ratios are 1..=NUM_PARTIALS.
pub const NUM_PARTIALS: usize = 8;

// Default patch: sawtooth-ish, a_n = 1/n.
pub const DEFAULT_AMPLITUDES: [f32; NUM_PARTIALS] = [
    1.0,
    1.0 / 2.0,
    1.0 / 3.0,
    1.0 / 4.0,
    1.0 / 5.0,
    1.0 / 6.0,
    1.0 / 7.0,
    1.0 / 8.0,
];

pub struct AdditiveSynth {
    wavetable: WaveTable,
}

fn generate_wave_table(
    frequency: f32,
    sample_rate: u16,
    amplitude: f32,
    phase: u8,
    partial_amplitudes: &[f32; NUM_PARTIALS],
) -> WaveTable {
    // Match Sine's (buggy) length formula so additive and sine tune identically.
    // Pre-3 will address the truncation/aliasing fix in wave_table.rs for both.
    let table_length = (sample_rate / frequency as u16 * 2) as usize;
    let samples_per_period = (sample_rate / frequency as u16) as f32;
    let phi = phase as f32 / 256.0 * 2.0 * std::f32::consts::PI;

    let nyquist = sample_rate as f32 / 2.0;

    let mut samples = vec![0f32; table_length];
    for (n_minus_one, a) in partial_amplitudes.iter().enumerate() {
        let n = (n_minus_one + 1) as f32;
        let partial_freq = n * frequency;
        if partial_freq >= nyquist {
            // Skip aliasing partials.
            continue;
        }
        if *a == 0.0 {
            continue;
        }
        for i in 0..table_length {
            samples[i] += a
                * (2.0 * std::f32::consts::PI * n * i as f32 / samples_per_period + phi).sin();
        }
    }

    // Peak-normalize to 1.0, then scale by global amplitude.
    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    if peak > 0.0 {
        let scale = amplitude / peak;
        for v in samples.iter_mut() {
            *v *= scale;
        }
    }

    WaveTable::new(samples, 0)
}

impl AdditiveSynth {
    pub fn new(frequency: f32, sample_rate: u16, amplitude: f32, phase: u8) -> AdditiveSynth {
        let wt = generate_wave_table(
            frequency,
            sample_rate,
            amplitude,
            phase,
            &DEFAULT_AMPLITUDES,
        );
        AdditiveSynth { wavetable: wt }
    }
}

impl Voice for AdditiveSynth {
    fn get_wavetable(self) -> WaveTable {
        self.wavetable
    }
}
