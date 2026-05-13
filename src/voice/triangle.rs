use crate::wave_table::{WaveTable, PERIOD_SAMPLES};
use crate::voice::Voice;

/// Band-limited triangle wave.
///
/// Fourier expansion: (8/π²) Σ_{n=1,3,5,..} (-1)^((n-1)/2) sin(nθ)/n².
/// Odd harmonics only with alternating sign and 1/n² falloff (much
/// softer than square). Stop at the highest odd n below Nyquist;
/// peak-normalize; scale by amplitude.
pub struct Triangle { wavetable: WaveTable }

fn generate(frequency: f32, sample_rate: u16, amplitude: f32, phase: u8) -> WaveTable {
    let len = PERIOD_SAMPLES;
    let phi = phase as f32 / 256.0 * 2.0 * std::f32::consts::PI;
    let nyquist = sample_rate as f32 / 2.0;
    let max_n = ((nyquist / frequency).floor() as u32).max(1);

    let mut samples = vec![0f32; len];
    let mut n = 1u32;
    let mut sign = 1.0f32;
    while n <= max_n {
	let coef = sign / (n as f32 * n as f32);
	let nf = n as f32;
	for i in 0..len {
	    samples[i] += coef
		* (2.0 * std::f32::consts::PI * nf * i as f32 / len as f32 + phi).sin();
	}
	n += 2;
	sign = -sign;
    }

    let peak = samples.iter().fold(0.0f32, |m, v| m.max(v.abs()));
    if peak > 0.0 {
	let scale = amplitude / peak;
	for v in samples.iter_mut() {
	    *v *= scale;
	}
    }
    WaveTable::new(samples, frequency, sample_rate)
}

impl Triangle {
    pub fn new(frequency: f32, sample_rate: u16, amplitude: f32, phase: u8) -> Triangle {
	Triangle { wavetable: generate(frequency, sample_rate, amplitude, phase) }
    }
}

impl Voice for Triangle {
    fn get_wavetable(self) -> WaveTable { self.wavetable }
}
