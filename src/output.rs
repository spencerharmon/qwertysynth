use std::{thread, time};
use crossbeam_channel::*;

use crate::sound_test;
pub struct Output;

impl Output {
    pub fn new() -> Output {
	Output { }
    }
    pub fn jack_output() {
        let (buffer_L_tx, buffer_L_rx) = unbounded();
        let (buffer_R_tx, buffer_R_rx) = unbounded();
        let (client, _status) =
            jack::Client::new("qwertysynth", jack::ClientOptions::NO_START_SERVER).unwrap();
        let mut left = client
            .register_port("left", jack::AudioOut::default())
            .unwrap();
        let mut right = client
            .register_port("right", jack::AudioOut::default())
            .unwrap();
        let process = jack::ClosureProcessHandler::new(
            move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
		
                // Get output buffer
                let out_l = left.as_mut_slice(ps);
                let out_r = right.as_mut_slice(ps);

                // Write output left
                for v in out_l.iter_mut() {
		    *v = 0.0;
		    if let Ok(float) = buffer_L_rx.try_recv() {
                        *v = float;
		    }
                }
    
                // Write output right
                for v in out_r.iter_mut() {
		    *v = 0.0;
		    if let Ok(float) = buffer_R_rx.try_recv() {
                        *v = float;
		    }
                }
                // Continue as normal
                jack::Control::Continue
            },
        );

        let active_client = client.activate_async((), process).unwrap();
	println!("end of function jack loop");

	sound_test::play_chord(buffer_L_tx, buffer_R_tx);
    }
}
