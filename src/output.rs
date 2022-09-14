use std::{thread, time};
use tokio::task;
use tokio::time::{sleep, Duration};
use crossbeam_channel::*;

use crate::sound_test;
use crate::keyboard;
use crate::wave_table::WaveTable;

pub struct Output;

impl Output {
    pub fn new() -> Output {
        Output { }
    }
    pub async fn jack_output(base_freq: f32, subdivisions: u8) {
        let (buffer_L_tx, buffer_L_rx) = bounded(1000);
        let (buffer_R_tx, buffer_R_rx) = bounded(1000);
	
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

	//key buffers
	let (key_on_tx, key_on_rx) = bounded(100 as usize);
	let (key_off_tx, key_off_rx) = bounded(100 as usize);
	let key_fut = keyboard::create_keyboard_listener(key_on_tx, key_off_tx);


	let instrument = sound_test::get_instrument(base_freq, subdivisions);
	instrument.play(key_on_rx, key_off_rx, buffer_L_tx, buffer_R_tx).await;
    }
}
