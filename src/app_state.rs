use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crate::envelope::EnvelopeParams;
use crate::voice::VoiceList;
pub use crate::voice::params::VoiceParams;

#[derive(Clone, Copy, Debug)]
pub struct EtParams {
    pub base_freq: f32,
    pub subdivisions: u8,
    pub multiplier: u8,
}

#[derive(Clone, Debug)]
pub enum TuningSystemList {
    EqualTemperment(EtParams),
}

impl TuningSystemList {
    pub fn name(&self) -> &'static str {
	match self {
	    TuningSystemList::EqualTemperment(_) => "equal temperment",
	}
    }
}

pub struct AppState {
    pub pressed: HashSet<u16>,
    pub jack_active: bool,
    pub current_voice: VoiceList,
    pub voice_params: VoiceParams,
    pub current_tuning: TuningSystemList,
    pub envelope_params: EnvelopeParams,
    pub last_key: Option<(char, f32)>,
    pub scale_freqs: Vec<f32>,
    pub show_voice_config: bool,
    pub show_tuning_config: bool,
}

impl AppState {
    pub fn new(
	voice: VoiceList,
	tuning: TuningSystemList,
	envelope_params: EnvelopeParams,
	scale_freqs: Vec<f32>,
    ) -> Self {
	Self {
	    pressed: HashSet::new(),
	    jack_active: false,
	    current_voice: voice,
	    voice_params: VoiceParams::default(),
	    current_tuning: tuning,
	    envelope_params,
	    last_key: None,
	    scale_freqs,
	    show_voice_config: false,
	    show_tuning_config: false,
	}
    }
}

pub type SharedState = Arc<Mutex<AppState>>;

/// Maps scale index (0..40) to the physical QWERTY glyph it lives on.
/// Sequence: zaq1 xsw2 cde3 vfr4 bgt5 nhy6 mju7 ,ki8 .lo9 /;p0
pub const KEY_GLYPHS: [char; 40] = [
    'z','a','q','1','x','s','w','2','c','d',
    'e','3','v','f','r','4','b','g','t','5',
    'n','h','y','6','m','j','u','7',',','k',
    'i','8','.','l','o','9','/',';','p','0',
];
