use crossbeam_channel::*;

use crate::equal_temperment;
use crate::wave_table;
use crate::scale;
use crate::instrument;
use crate::polysynth;
use ndarray::{arr1, Array, Dim};

pub fn get_instrument_with_base_freq(freq: f32) -> instrument::Instrument{
        let et = equal_temperment::EqualTemperment::new(
            freq,
            equal_temperment::DEFAULT_SUBDIVISIONS,
            equal_temperment::DEFAULT_OCTAVES,
            equal_temperment::DEFAULT_MULTIPLIER,
        );
    
        let scale = et.generate_scale();
    
        let mut scale_wave_tables: Vec<wave_table::WaveTable> = Vec::new();
            
        for f in scale.get_frequencies_vector() {
            let wtg = wave_table::WaveTableGenerator::new(
                f,
                wave_table::DEFAULT_SAMPLE_RATE,
                wave_table::DEFAULT_AMPLITUDE,
                wave_table::DEFAULT_PHASE,
            );
            let mut wt = wtg.generate_wave_table_sine();
            scale_wave_tables.push(wt);
        }
        let mut instrument = instrument::Instrument::new(scale_wave_tables);

    return instrument;
}
pub struct DefaultTestInstrument {
    pub instrument: instrument::Instrument
}
impl DefaultTestInstrument {
    pub fn new() -> DefaultTestInstrument{
	let instrument = get_instrument_with_base_freq(
	    equal_temperment::DEFAULT_BASE_FREQUENCY
	);
	DefaultTestInstrument { instrument }
    }
}
pub fn play_scale(out_L: Sender<f32>, out_R: Sender<f32>){
    let mut instrument = DefaultTestInstrument::new();
    loop {
        for t in &mut instrument.instrument.scale_wave_tables {
	    for _i in 0..10000 {
		out_L.send(t.next()).unwrap();
		out_R.send(t.next()).unwrap();
	    }
	}
    } 
}

pub fn play_chord(out_L: Sender<f32>, out_R: Sender<f32>){
    let instrument = DefaultTestInstrument::new();
    let note1 = &instrument.instrument.scale_wave_tables[22];
    let note2 = &instrument.instrument.scale_wave_tables[26];
    let note3 = &instrument.instrument.scale_wave_tables[29];
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
