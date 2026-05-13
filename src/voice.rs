pub mod sine;
pub mod additive_synth;
pub mod params;

use crate::voice::sine::Sine;
use crate::voice::additive_synth::AdditiveSynth;
use crate::voice::params::VoiceParams;
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
    AdditiveSynth,
}

impl FromStr for VoiceList {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, io::Error> {
	match s.to_ascii_lowercase().as_str() {
	    "sine" => Ok(VoiceList::Sine),
	    "additive-synth" => Ok(VoiceList::AdditiveSynth),
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
            Self::AdditiveSynth =>
		AdditiveSynth::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
        }
    }

    pub fn get_wavetable_with_params(
	&self,
	frequency: f32,
	sample_rate: u16,
	params: &VoiceParams,
    ) -> WaveTable {
	match self {
	    Self::Sine =>
		Sine::new(frequency, sample_rate, params.amplitude, params.phase)
		.get_wavetable(),
	    Self::AdditiveSynth =>
		AdditiveSynth::with_partials(
		    frequency,
		    sample_rate,
		    params.amplitude,
		    params.phase,
		    &params.partial_amplitudes,
		    &params.partial_phases,
		)
		.get_wavetable(),
	}
    }
}
    
