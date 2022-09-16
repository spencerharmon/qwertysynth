pub const DEFAULT_BASE_FREQUENCY:f32 = 440.0;
pub const DEFAULT_SAMPLE_RATE:u16 = 48000;
pub const DEFAULT_AMPLITUDE:f32 = 0.8;
pub const DEFAULT_PHASE:u8 = 0;

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

