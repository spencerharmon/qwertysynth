pub mod sine;
pub mod additive_synth;
pub mod sawtooth;
pub mod square;
pub mod triangle;
pub mod pwm;
pub mod params;

use crate::voice::sine::Sine;
use crate::voice::additive_synth::AdditiveSynth;
use crate::voice::sawtooth::Sawtooth;
use crate::voice::square::Square;
use crate::voice::triangle::Triangle;
use crate::voice::pwm::Pwm;
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
    Sawtooth,
    Square,
    Triangle,
    Pwm,
}

impl FromStr for VoiceList {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, io::Error> {
	match s.to_ascii_lowercase().as_str() {
	    "sine" => Ok(VoiceList::Sine),
	    "additive-synth" => Ok(VoiceList::AdditiveSynth),
	    "sawtooth" => Ok(VoiceList::Sawtooth),
	    "square" => Ok(VoiceList::Square),
	    "triangle" => Ok(VoiceList::Triangle),
	    "pwm" => Ok(VoiceList::Pwm),
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
            Self::Sawtooth =>
		Sawtooth::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
            Self::Square =>
		Square::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
            Self::Triangle =>
		Triangle::new(frequency, sample_rate, amplitude, phase)
		.get_wavetable(),
            Self::Pwm =>
		Pwm::new(frequency, sample_rate, amplitude, phase, 0.5)
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
	    Self::Sawtooth =>
		Sawtooth::new(frequency, sample_rate, params.amplitude, params.phase)
		.get_wavetable(),
	    Self::Square =>
		Square::new(frequency, sample_rate, params.amplitude, params.phase)
		.get_wavetable(),
	    Self::Triangle =>
		Triangle::new(frequency, sample_rate, params.amplitude, params.phase)
		.get_wavetable(),
	    Self::Pwm =>
		Pwm::new(
		    frequency,
		    sample_rate,
		    params.amplitude,
		    params.phase,
		    params.pwm_duty,
		)
		.get_wavetable(),
	}
    }
}
