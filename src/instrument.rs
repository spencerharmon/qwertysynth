use tokio::task;
use crossbeam_channel::*;
use crate::wave_table::WaveTable;

pub struct Instrument {
    pub scale_wave_tables: Vec<WaveTable>
}

impl Instrument {
    pub fn new(scale_wave_tables: Vec<WaveTable>) -> Instrument {
	Instrument { scale_wave_tables: scale_wave_tables }
    }
    pub fn play(
	self,
	key_on: Receiver<u16>,
	key_off: Receiver<u16>,
	out_l: Sender<f32>,
	out_r: Sender<f32>
    ) -> task::JoinHandle<()> {

	let fut = task::spawn(async move {
            let mut chord: Vec<&WaveTable> = Vec::new();
            let mut index = 0;
            loop {
                if let Ok(val) = key_off.try_recv() {
		    chord.retain(|x| *x != &self.scale_wave_tables[val as usize]);
		}
		if let Ok(val) = key_on.try_recv() {
		    chord.push(&self.scale_wave_tables[val as usize]);
		}
                let mut sample = 0.0;

		for note in &chord {
		    sample = sample + note.wavetable[index % note.wavetable.len()];
		}
        	out_l.send(sample);
        	out_r.send(sample);
        	index = index + 1;
	    }
	    
	});
	fut
    }
}
