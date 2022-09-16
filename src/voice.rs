pub mod sine;
pub mod test_additive_synth;

use crate::voice::sine::Sine;
use crate::voice::test_additive_synth::TestAdditiveSynth;
use crate::wave_table::WaveTable;

use std::str::FromStr;
use std::io;
use clap::ArgEnum;

pub trait Voice {
    fn get_wavetable(self) -> WaveTable;
}

#[derive(Clone, ArgEnum)]
pub enum VoiceList {
    Sine,
    TestAdditiveSynth,
}

impl FromStr for VoiceList {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, io::Error> {
	match s.to_ascii_lowercase().as_str() {
	    "sine" => Ok(VoiceList::Sine),
	    "test-additive-synth" => Ok(VoiceList::TestAdditiveSynth),
	    _ => return Err(std::io::Error::new(io::ErrorKind::Other, "invalid voice")),
	}
    }
}

impl VoiceList {
    pub fn get_wavetable(&self,
		 frequency: f32,
		 sample_rate: u16,
		 amplitude: f32,
			 phase: u8) -> WaveTable {
	
        match self {
            Self::Sine =>
		Sine::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
            Self::TestAdditiveSynth =>
		TestAdditiveSynth::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
        }
    }
}
    
