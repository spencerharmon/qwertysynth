pub const DEFAULT_BASE_FREQUENCY:f32 = 440.0;
pub const DEFAULT_SAMPLE_RATE:u16 = 48000;
pub const DEFAULT_AMPLITUDE:f32 = 0.8;
pub const DEFAULT_PHASE:u8 = 0;

/// Length of one canonical period in samples. Voices generate exactly
/// one period of this length and the playback loop steps through it
/// at a fractional rate determined by note frequency, so pitch is no
/// longer baked into the table size. 2048 is a power of two with
/// sub-cent linear-interpolation error across the audible range.
pub const PERIOD_SAMPLES: usize = 2048;

#[derive(Clone)]
pub struct WaveTable {
    pub wavetable: Vec<f32>,
    phase: f32,
    step: f32,
}

impl WaveTable {
    pub fn new(wavetable: Vec<f32>, frequency: f32, sample_rate: u16) -> WaveTable {
	let len = wavetable.len() as f32;
	let step = if len > 0.0 {
	    frequency * len / sample_rate as f32
	} else {
	    0.0
	};
	WaveTable {
	    wavetable,
	    phase: 0.0,
	    step,
	}
    }

    pub fn next(&mut self) -> f32 {
	let len = self.wavetable.len();
	if len == 0 {
	    return 0.0;
	}
	let i0 = self.phase as usize % len;
	let i1 = (i0 + 1) % len;
	let frac = self.phase - self.phase.floor();
	let s0 = self.wavetable[i0];
	let s1 = self.wavetable[i1];
	let sample = s0 + (s1 - s0) * frac;

	self.phase += self.step;
	let len_f = len as f32;
	while self.phase >= len_f {
	    self.phase -= len_f;
	}
	sample
    }
}

impl PartialEq for WaveTable {
    fn eq(&self, other: &Self) -> bool {
	self.wavetable == other.wavetable
    }
}
