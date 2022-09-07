use crate::wave_table::*;
use crossbeam_channel::*;
use ndarray::{arr1, Array};

pub struct Chord {
    notes: Vec<&'static WaveTable>,
}

impl Chord {
    pub fn new(notes: Vec<&'static WaveTable>) -> Chord {
	Chord { notes }
    }
    pub fn output_to_buffer(self, out_L: Sender<f32>, out_R: Sender<f32>){
    }
}
