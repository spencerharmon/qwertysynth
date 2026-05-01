use crate::wave_table::WaveTable;
use crate::voice::Voice;

pub struct Sine {
    wavetable: WaveTable
}

fn generate_wave_table(frequency: f32,
		       sample_rate: u16,
		       amplitude: f32,
		       phase: u8) -> WaveTable {
    // this is not the best way to do this. The table becomes truncated due to
    // casting float as u16. We should find a multiple that is as close as possible
    // to evenly divisible by 1 and use this length to avoid creating artifacts in
    // the signal.
    // TODO. (tracked as a post-GUI item in plan.org)
    let table_length =  sample_rate / frequency as u16 * 2;
    let samples_per_period = (sample_rate / frequency as u16) as f32;
    let phi = phase as f32 / 256.0 * 2.0 * std::f32::consts::PI;

    let mut wavetable = vec![0f32; table_length as usize];
    for i in 0..table_length as usize {
        wavetable[i] = amplitude
            * (2.0 * std::f32::consts::PI * i as f32 / samples_per_period + phi).sin();
    }
    WaveTable::new(
	wavetable,
	0
    )
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
