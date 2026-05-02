#[derive(Clone, Copy)]
pub struct EnvelopeParams {
    pub attack_s: f32,
    pub decay_s: f32,
    pub sustain: f32,
    pub release_s: f32,
}

impl EnvelopeParams {
    pub const DEFAULT_ATTACK_S: f32 = 0.005;
    pub const DEFAULT_DECAY_S: f32 = 0.050;
    pub const DEFAULT_SUSTAIN: f32 = 0.8;
    pub const DEFAULT_RELEASE_S: f32 = 0.050;
}

#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

#[derive(Clone)]
pub struct Envelope {
    stage: Stage,
    amp: f32,
    sustain: f32,
    attack_step: f32,
    decay_step: f32,
    release_step: f32,
}

impl Envelope {
    pub fn new(params: EnvelopeParams, sample_rate: u16) -> Envelope {
	let sr = sample_rate as f32;
	let attack_step = if params.attack_s > 0.0 { 1.0 / (params.attack_s * sr) } else { f32::INFINITY };
	let decay_step = if params.decay_s > 0.0 {
	    (1.0 - params.sustain) / (params.decay_s * sr)
	} else {
	    f32::INFINITY
	};
	let release_step = if params.release_s > 0.0 { 1.0 / (params.release_s * sr) } else { f32::INFINITY };
	Envelope {
	    stage: Stage::Attack,
	    amp: 0.0,
	    sustain: params.sustain.clamp(0.0, 1.0),
	    attack_step,
	    decay_step,
	    release_step,
	}
    }

    pub fn release(&mut self) {
	if self.stage != Stage::Finished {
	    self.stage = Stage::Release;
	}
    }

    pub fn is_finished(&self) -> bool {
	self.stage == Stage::Finished
    }

    /// True while the note is still responding to key-down (i.e. not
    /// yet released). key_off should target one of these, not a voice
    /// that's already releasing or finished.
    pub fn is_held(&self) -> bool {
	matches!(self.stage, Stage::Attack | Stage::Decay | Stage::Sustain)
    }

    pub fn next(&mut self) -> f32 {
	match self.stage {
	    Stage::Attack => {
		self.amp += self.attack_step;
		if self.amp >= 1.0 {
		    self.amp = 1.0;
		    self.stage = Stage::Decay;
		}
	    }
	    Stage::Decay => {
		self.amp -= self.decay_step;
		if self.amp <= self.sustain {
		    self.amp = self.sustain;
		    self.stage = Stage::Sustain;
		}
	    }
	    Stage::Sustain => {
		self.amp = self.sustain;
	    }
	    Stage::Release => {
		self.amp -= self.release_step;
		if self.amp <= 0.0 {
		    self.amp = 0.0;
		    self.stage = Stage::Finished;
		}
	    }
	    Stage::Finished => {
		self.amp = 0.0;
	    }
	}
	self.amp
    }
}
