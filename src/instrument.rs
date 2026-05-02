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
	mut self,
	key_on: Receiver<u16>,
	key_off: Receiver<u16>,
	swap_in: Receiver<Vec<WaveTable>>,
	out_l: Sender<f32>,
	out_r: Sender<f32>
    ) -> task::JoinHandle<()> {

	let fut = task::spawn(async move {
            // Track each active voice by its scale index. With pitch
            // now living in WaveTable's step rather than its sample
            // buffer, two notes of the same voice have identical
            // buffers and can't be told apart by content; the index
            // is what distinguishes them on key-off.
            let mut chord: Vec<(u16, WaveTable)> = Vec::new();
            loop {
                if let Ok(new_tables) = swap_in.try_recv() {
                    self.scale_wave_tables = new_tables;
                    chord.clear();
                }
                if let Ok(val) = key_off.try_recv() {
		    if let Some(pos) = chord.iter().position(|(idx, _)| *idx == val) {
			chord.remove(pos);
		    }
		}
		if let Ok(val) = key_on.try_recv() {
		    chord.push((val, self.scale_wave_tables[val as usize].clone()));
		}
                let mut sample = 0.0;
		for (_, note) in chord.iter_mut() {
		    sample += note.next();
		}
        	out_l.send(sample);
        	out_r.send(sample);
	    }
	    
	});
	fut
    }
}
