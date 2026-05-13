use crate::voice::additive_synth::{DEFAULT_AMPLITUDES, NUM_PARTIALS};
use crate::wave_table::{DEFAULT_AMPLITUDE, DEFAULT_PHASE};

#[derive(Clone, Debug)]
pub struct VoiceParams {
    pub amplitude: f32,
    pub phase: u8,
    pub partial_amplitudes: [f32; NUM_PARTIALS],
    pub partial_phases: [f32; NUM_PARTIALS],
    pub pwm_duty: f32,
}

impl Default for VoiceParams {
    fn default() -> Self {
	Self {
	    amplitude: DEFAULT_AMPLITUDE,
	    phase: DEFAULT_PHASE,
	    partial_amplitudes: DEFAULT_AMPLITUDES,
	    partial_phases: [0.0; NUM_PARTIALS],
	    pwm_duty: 0.5,
	}
    }
}
