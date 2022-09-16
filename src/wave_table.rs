pub const DEFAULT_BASE_FREQUENCY:f32 = 440.0;
pub const DEFAULT_SAMPLE_RATE:u16 = 48000;
pub const DEFAULT_AMPLITUDE:f32 = 0.8;
pub const DEFAULT_PHASE:u8 = 0;

pub struct WaveTableGenerator {
    frequency: f32,
    sample_rate: u16,
    amplitude: f32,
    phase: u8
}

impl WaveTableGenerator {
    pub fn new(frequency: f32,
               sample_rate: u16,
               amplitude: f32,
               phase: u8) -> WaveTableGenerator {
	WaveTableGenerator {
	    frequency: frequency,
	    sample_rate: sample_rate,
	    amplitude: amplitude,
	    phase: phase
	}
    }

    pub fn generate_wave_table_sine(self) -> WaveTable {
	// this is not the best way to do this. The table becomes truncated due to
	// casting float as u16. We should find a multiple that is as close as possible
	// to evenly divisible by 1 and use this length to avoid creating artifacts in
	// the signal.
	// TODO.
	let table_length =  self.sample_rate / self.frequency as u16 * 2;
	let mut wavetable = sine_wave_generator(
	    &self.frequency,
	    table_length as usize,
	    self.sample_rate
	);
	WaveTable::new(
	    wavetable,
	    0
	)
    }
    pub fn generate_wave_table_with_harmonics(self) -> WaveTable {
	// this is not the best way to do this. The table becomes truncated due to
	// casting float as u16. We should find a multiple that is as close as possible
	// to evenly divisible by 1 and use this length to avoid creating artifacts in
	// the signal.
	// TODO.
	let table_length =  self.sample_rate / self.frequency as u16 * 8u16;
	let mut wavetable0 = sine_wave_generator(
	    &self.frequency,
	    table_length as usize,
	    self.sample_rate
	);
	let mut wavetable1 = sine_wave_generator(
	    &(self.frequency * 0.5),
	    table_length as usize,
	    self.sample_rate
	);
	let mut wavetable2 = sine_wave_generator(
	    &(self.frequency * 2.0/3.0),
	    table_length as usize,
	    self.sample_rate
	);
	let mut wavetable3 = sine_wave_generator(
	    &(self.frequency * 3.0/2.0),
	    table_length as usize,
	    self.sample_rate
	);

	let mut wavetable7 = sine_wave_generator(
	    &((self.frequency * 3.0/2.0) + 35.0),
	    table_length as usize,
	    self.sample_rate
	);

	let mut wavetable4 = sine_wave_generator(
	    &(self.frequency * 5.0/8.0),
	    table_length as usize,
	    self.sample_rate
	);
	
	let mut wavetable5 = sine_wave_generator(
	    &(self.frequency + 15_000.0),
	    table_length as usize,
	    self.sample_rate
	);
	
	let mut wavetable6 = sine_wave_generator(
	    &(self.frequency / 8.0),
	    table_length as usize,
	    self.sample_rate
	);
	

	let mut wavetable = vec![0f32; table_length.into()];
	for i in 0..table_length {
	    wavetable[i as usize] = wavetable0[i as usize] + 
		wavetable1[i as usize] * 0.6 + 
		wavetable2[i as usize] * 0.3 +
		wavetable3[i as usize] * 0.5 +
		wavetable4[i as usize] * 0.3 +
		wavetable5[i as usize] * 0.5 +
		wavetable7[i as usize] * 0.5 +
		wavetable6[i as usize] * 0.8;

		
	}
	WaveTable::new(
	    wavetable,
	    0
	)
    }
								  
}
pub fn sine_wave_generator(freq: &f32, length: usize, sample_rate: u16) -> Vec<f32> {
    let mut ret = vec![0f32; length.into()];
    let samples_per_period =  sample_rate / *freq as u16;
    for i in 0..length {
        ret[i as usize] = (2f32 * std::f32::consts::PI * i as f32 / samples_per_period as f32).sin();

    }
	ret
}


#[derive(Clone)]
pub struct WaveTable {
    pub wavetable: Vec<f32>,
    index: u16
}

impl WaveTable {
    pub fn new(wavetable: Vec<f32>,
	       index: u16) -> WaveTable {
	WaveTable {
	    wavetable: wavetable,
	    index: index
	}
    }
    pub fn next(&mut self) -> f32 {
	if self.index == self.wavetable.capacity() as u16 {
	    self.index = 0;
	}
	let ret = self.wavetable[self.index as usize];
	self.index = self.index + 1u16;
	return ret;
    }
}

impl PartialEq for WaveTable {
    fn eq(&self, other: &Self) -> bool {
	self.wavetable == other.wavetable
    }
}

//impl Copy for WaveTable {
//    fn copy(&self) -> WaveTable {
//	WaveTable { self.wavetable, 0u16 }
//    }
//}
pub struct WaveTableScale {
    tables: Vec<WaveTable>,
}

impl WaveTableScale {
    fn new (tables: Vec<WaveTable>) -> WaveTableScale {
	WaveTableScale {
	    tables: tables
	}
    }
}

