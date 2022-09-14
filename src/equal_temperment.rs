use crate::scale;

pub const DEFAULT_BASE_FREQUENCY: f32 = 440.0;
pub const DEFAULT_SUBDIVISIONS: u8 = 33;
pub const DEFAULT_OCTAVES: u8 = 6;
pub const DEFAULT_MULTIPLIER: u8 = 2;

pub struct EqualTemperment {
    base_frequency: f32,
    subdivisions: u8,
    octaves: u8,
    multiplier: u8,
}

impl EqualTemperment {
    pub fn new(base_frequency: f32,
	   subdivisions: u8,
	   octaves: u8,
	   multiplier: u8) -> EqualTemperment {
	EqualTemperment {
	    base_frequency: base_frequency,
	    subdivisions: subdivisions,
	    octaves: octaves,
	    multiplier: multiplier
	}
    }
    pub fn generate_scale(self) -> scale::Scale {
	let mut bottom_freq: f32 = self.base_frequency;
	let mut top_freq: f32 = bottom_freq * self.multiplier as f32;;
	let mut scale_frequencies:Vec<f32> = Vec::new();
	while scale_frequencies.len() <= 40{
	    let diff = top_freq - bottom_freq;
	    let interval = diff/self.subdivisions as f32;
	    scale_frequencies.push(bottom_freq);
	    let mut next_freq: f32 = bottom_freq;
	    for _x in 0..self.subdivisions {
		next_freq = next_freq + interval;
		scale_frequencies.push(next_freq);
	    }
	    scale_frequencies.pop();
	    bottom_freq = top_freq;
	    top_freq = bottom_freq * self.multiplier as f32;
	}
	scale::Scale::new(scale_frequencies)
    }
}
