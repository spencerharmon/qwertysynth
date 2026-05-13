use tokio::task;
use crossbeam_channel::*;
use crate::wave_table::WaveTable;
use crate::envelope::{Envelope, EnvelopeParams};
use crate::keyboard::{KEY_EVENT_LEFT_SHIFT, KEY_EVENT_RIGHT_SHIFT};

struct ActiveVoice {
    idx: u16,
    table: WaveTable,
    env: Envelope,
    /// True iff a key_off arrived for this voice while the sustain
    /// pedal was held. Released en masse on pedal-up.
    release_pending: bool,
}

pub struct Instrument {
    pub scale_wave_tables: Vec<WaveTable>,
    envelope_params: EnvelopeParams,
    sample_rate: u16,
}

impl Instrument {
    pub fn new(
	scale_wave_tables: Vec<WaveTable>,
	envelope_params: EnvelopeParams,
	sample_rate: u16,
    ) -> Instrument {
	Instrument {
	    scale_wave_tables,
	    envelope_params,
	    sample_rate,
	}
    }
    pub fn play(
	mut self,
	key_on: Receiver<u16>,
	key_off: Receiver<u16>,
	swap_in: Receiver<Vec<WaveTable>>,
	env_in: Receiver<EnvelopeParams>,
	midi_sustain_in: Receiver<bool>,
	out_l: Sender<f32>,
	out_r: Sender<f32>
    ) -> task::JoinHandle<()> {

	let fut = task::spawn(async move {
            let mut chord: Vec<ActiveVoice> = Vec::new();
	    // Sustain pedal sources tracked independently. Effective
	    // pedal state is the OR.
	    let mut left_shift = false;
	    let mut right_shift = false;
	    let mut midi_sustain = false;
	    let mut sustain_held = false;
            loop {
                while let Ok(new_tables) = swap_in.try_recv() {
                    self.scale_wave_tables = new_tables;
                    chord.clear();
                }
                while let Ok(new_env) = env_in.try_recv() {
                    // Future notes only; in-flight envelopes keep
                    // their original step rates to avoid amp jumps.
                    self.envelope_params = new_env;
                }
                while let Ok(v) = midi_sustain_in.try_recv() {
                    midi_sustain = v;
                }
                // Drain all pending presses first, then all pending
                // releases. Doing only one per sample lets fast
                // playing back up the channels and strands notes when
                // the same key is re-pressed during its own release.
                while let Ok(val) = key_on.try_recv() {
		    match val {
			KEY_EVENT_LEFT_SHIFT => { left_shift = true; }
			KEY_EVENT_RIGHT_SHIFT => { right_shift = true; }
			_ => {
			    // Re-press during pedal: any pedal-deferred
			    // voice for this scale index gets released
			    // now so it doesn't stack with the new
			    // voice. Otherwise N repeated presses with
			    // sustain held would sum N in-phase copies.
			    for v in chord.iter_mut() {
				if v.idx == val && v.release_pending {
				    v.env.release();
				    v.release_pending = false;
				}
			    }
			    let env = Envelope::new(self.envelope_params, self.sample_rate);
			    chord.push(ActiveVoice {
				idx: val,
				table: self.scale_wave_tables[val as usize].clone(),
				env,
				release_pending: false,
			    });
			}
		    }
                }
                while let Ok(val) = key_off.try_recv() {
		    match val {
			KEY_EVENT_LEFT_SHIFT => { left_shift = false; }
			KEY_EVENT_RIGHT_SHIFT => { right_shift = false; }
			_ => {
			    if let Some(pos) = chord.iter()
				.position(|v| v.idx == val && v.env.is_held() && !v.release_pending)
			    {
				if sustain_held {
				    chord[pos].release_pending = true;
				} else {
				    chord[pos].env.release();
				}
			    }
			}
		    }
		}
		// Recompute sustain after draining all sources; on
		// the held -> released transition, fire every
		// release_pending voice at once.
		let new_sustain = left_shift || right_shift || midi_sustain;
		if sustain_held && !new_sustain {
		    for v in chord.iter_mut() {
			if v.release_pending {
			    v.env.release();
			    v.release_pending = false;
			}
		    }
		}
		sustain_held = new_sustain;

                let mut sample = 0.0;
		for v in chord.iter_mut() {
		    sample += v.table.next() * v.env.next();
		}
		chord.retain(|v| !v.env.is_finished());
        	out_l.send(sample);
        	out_r.send(sample);
	    }

	});
	fut
    }
}
