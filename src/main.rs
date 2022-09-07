use std::{thread, time};
use crossbeam_channel::*;
mod equal_temperment;
mod wave_table;
mod scale;
mod instrument;
mod output;
mod sound_test;
mod polysynth;

fn main() {
    
    let (buffer_L_tx, buffer_L_rx) = unbounded();
    let (buffer_R_tx, buffer_R_rx) = unbounded();
//    let out_L = buffer_L_rx.clone();
//    let out_R = buffer_R_rx.clone();
    let output = output::Output::new();
    
    output::Output::jack_output(&buffer_L_rx, &buffer_R_rx);
    let result = buffer_L_tx.send(0.0);

    //prints "sending on a disconnected channel" unless clones are used, as above.
    match result {
	Ok(_) => (),
	Err(e) => { println!("{}", e) }
	    
    }
    sound_test::play_chord(buffer_L_tx, buffer_R_tx);
}

