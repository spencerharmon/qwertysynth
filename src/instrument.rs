use tokio::task;
use crossbeam_channel::*;
use crate::wave_table::WaveTable;
use crate::envelope::{Envelope, EnvelopeParams};

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
	out_l: Sender<f32>,
	out_r: Sender<f32>
    ) -> task::JoinHandle<()> {

	let fut = task::spawn(async move {
            // (scale_index, wavetable, envelope). Multiple entries for
            // the same scale_index are allowed: re-pressing a key
            // during its own release tail spawns a new voice and lets
            // the old one finish. key_off targets the first
            // still-held (Attack/Decay/Sustain) voice for that index.
            let mut chord: Vec<(u16, WaveTable, Envelope)> = Vec::new();
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
                // Drain all pending presses first, then all pending
                // releases. Doing only one per sample lets fast
                // playing back up the channels and strands notes when
                // the same key is re-pressed during its own release.
                while let Ok(val) = key_on.try_recv() {
		    let env = Envelope::new(self.envelope_params, self.sample_rate);
		    chord.push((val, self.scale_wave_tables[val as usize].clone(), env));
                }
                while let Ok(val) = key_off.try_recv() {
		    if let Some(pos) = chord.iter()
			.position(|(idx, _, env)| *idx == val && env.is_held())
		    {
			chord[pos].2.release();
		    }
		}
                let mut sample = 0.0;
		for (_, note, env) in chord.iter_mut() {
		    sample += note.next() * env.next();
		}
		chord.retain(|(_, _, env)| !env.is_finished());
        	out_l.send(sample);
        	out_r.send(sample);
	    }

	});
	fut
    }
}
