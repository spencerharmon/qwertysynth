use std::{thread, time};
use tokio::task;
use tokio::time::{sleep, Duration};
use crossbeam_channel::*;

use crate::keyboard;
use crate::midi;
use crate::wave_table::WaveTable;
use crate::envelope::EnvelopeParams;
use crate::instrument::Instrument;
pub struct Output;

impl Output {
    pub fn new() -> Output {
        Output { }
    }
    pub async fn jack_output(
	base_freq: f32,
	subdivisions: u8,
	instrument: Instrument,
	swap_in: Receiver<Vec<WaveTable>>,
	env_in: Receiver<EnvelopeParams>,
	midi_sustain_gui_tx: Sender<bool>,
    ) {
        let (buffer_L_tx, buffer_L_rx) = bounded(1000);
        let (buffer_R_tx, buffer_R_rx) = bounded(1000);

	// Local channel for the instrument-side MIDI sustain stream.
	// The JACK process callback runs on the realtime audio thread,
	// so it must never block; bounded + try_send keeps it RT-safe.
	// 64 entries is far more than the few sustain transitions per
	// second a human can generate.
	let (midi_sustain_local_tx, midi_sustain_local_rx) = bounded::<bool>(64);

        let (client, _status) =
            jack::Client::new("qwertysynth", jack::ClientOptions::NO_START_SERVER).unwrap();
        let mut left = client
            .register_port("left", jack::AudioOut::default())
            .unwrap();
        let mut right = client
            .register_port("right", jack::AudioOut::default())
            .unwrap();
        let sustain_port = client
            .register_port("sustain", jack::MidiIn::default())
            .unwrap();
        let process = jack::ClosureProcessHandler::new(
            move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {

                for ev in sustain_port.iter(ps) {
                    if let Some(down) = midi::parse_cc_sustain(ev.bytes) {
                        let _ = midi_sustain_local_tx.try_send(down);
                        let _ = midi_sustain_gui_tx.try_send(down);
                    }
                }

                let out_l = left.as_mut_slice(ps);
                let out_r = right.as_mut_slice(ps);

                for v in out_l.iter_mut() {
                    *v = 0.0;
                    if let Ok(float) = buffer_L_rx.try_recv() {
                        *v = float;
                    }
                }

                for v in out_r.iter_mut() {
                    *v = 0.0;
                    if let Ok(float) = buffer_R_rx.try_recv() {
                        *v = float;
                    }
                }
                jack::Control::Continue
            },
        );

        let active_client = client.activate_async((), process).unwrap();

	//key buffers
	let (key_on_tx, key_on_rx) = bounded(100 as usize);
	let (key_off_tx, key_off_rx) = bounded(100 as usize);
	let key_fut = keyboard::create_keyboard_listener(key_on_tx, key_off_tx);


	instrument.play(
	    key_on_rx,
	    key_off_rx,
	    swap_in,
	    env_in,
	    midi_sustain_local_rx,
	    buffer_L_tx,
	    buffer_R_tx,
	).await;
    }
}
