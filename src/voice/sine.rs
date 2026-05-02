use crate::wave_table::{WaveTable, PERIOD_SAMPLES};
use crate::voice::Voice;

pub struct Sine {
    wavetable: WaveTable
}

fn generate_wave_table(frequency: f32,
		       sample_rate: u16,
		       amplitude: f32,
		       phase: u8) -> WaveTable {
    let len = PERIOD_SAMPLES;
    let phi = phase as f32 / 256.0 * 2.0 * std::f32::consts::PI;

    let mut wavetable = vec![0f32; len];
    for i in 0..len {
        wavetable[i] = amplitude
            * (2.0 * std::f32::consts::PI * i as f32 / len as f32 + phi).sin();
    }
    WaveTable::new(wavetable, frequency, sample_rate)
}

impl Sine {
    pub fn new(frequency: f32,
               sample_rate: u16,
               amplitude: f32,
               phase: u8) -> Sine {

	let wt = generate_wave_table(frequency,
			    sample_rate,
			    amplitude,
			    phase);
	Sine{ wavetable: wt }
    }
}

impl Voice for Sine {
    fn get_wavetable(self) -> WaveTable {
	self.wavetable
    }
}
