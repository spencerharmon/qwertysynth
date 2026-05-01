use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crate::voice::VoiceList;

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
    pub current_tuning: TuningSystemList,
    pub last_key: Option<(char, f32)>,
    pub scale_freqs: Vec<f32>,
}

impl AppState {
    pub fn new(
	voice: VoiceList,
	tuning: TuningSystemList,
	scale_freqs: Vec<f32>,
    ) -> Self {
	Self {
	    pressed: HashSet::new(),
	    jack_active: false,
	    current_voice: voice,
	    current_tuning: tuning,
	    last_key: None,
	    scale_freqs,
	}
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
