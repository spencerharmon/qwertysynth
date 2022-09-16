use crossbeam_channel::*;

use crate::equal_temperment;
use crate::wave_table;
use crate::scale;
use crate::instrument;
use crate::polysynth;
use ndarray::{arr1, Array, Dim};

pub fn play_scale(out_L: Sender<f32>, out_R: Sender<f32>,
		  instrument: &mut instrument::Instrument){
    loop {
        for t in &mut instrument.scale_wave_tables {
	    for _i in 0..10000 {
		out_L.send(t.next()).unwrap();
		out_R.send(t.next()).unwrap();
	    }
	}
    } 
}

pub fn play_chord(out_L: Sender<f32>, out_R: Sender<f32>,
		  instrument: &mut instrument::Instrument){
    let note1 = &instrument.scale_wave_tables[22];
    let note2 = &instrument.scale_wave_tables[26];
    let note3 = &instrument.scale_wave_tables[29];
    let chord = vec![note1, note2, note3];
//    let mut out = arr1(&[ ]);

//	for wt in chord {
//	    let a: Array<f32, Dim<[usize; 1]>> = Array::from_iter(wt.wavetable.into_iter());
//	    out = out + a;
//	}
//    let mut wavetable = wave_table::WaveTable::new(out.into_raw_vec(), 0u16);
    let mut index = 0;
	loop {
	    let mut sample = 0.0;
	    
	    for note in &chord {
		sample = sample + note.wavetable[index % note.wavetable.len()];
	    }
	    out_L.send(sample);
	    out_R.send(sample);
	    index = index + 1;
	}
}
