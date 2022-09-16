use crate::wave_table:: { WaveTable, sine_wave_generator };
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
    // TODO.
    let table_length =  sample_rate / frequency as u16 * 2;
    let mut wavetable = sine_wave_generator(
	&frequency,
	table_length as usize,
	sample_rate
    );
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
