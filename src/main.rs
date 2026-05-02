use tokio;
use crossbeam_channel::*;
mod equal_temperment;
mod wave_table;
mod scale;
mod instrument;
mod output;
mod sound_test;
mod polysynth;
mod keyboard;
mod voice;
mod gui;
mod app_state;


use crate::voice::Voice;

use clap::Parser;
#[derive(Parser)]
struct Cli {
    #[clap(short='b', long="base_freq", default_value_t=wave_table::DEFAULT_BASE_FREQUENCY)]
    base_freq: f32,
    #[clap(short='s', long="subdivisions", default_value_t=equal_temperment::DEFAULT_SUBDIVISIONS)]
    subdivisions: u8,
    #[clap(arg_enum, long="voice", default_value="sine")]
    voice: voice::VoiceList,
}


fn main() {
    // keyboard_query reads via X11 XQueryKeymap. On Wayland sessions,
    // a Wayland-native eframe window doesn't share key state with
    // XWayland, so unset WAYLAND_DISPLAY to force eframe/winit to use
    // X11 (XWayland) — same display the keyboard listener reads.
    std::env::remove_var("WAYLAND_DISPLAY");
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    let args = Cli::parse();

    let et = equal_temperment::EqualTemperment::new(
        args.base_freq,
	args.subdivisions,
	equal_temperment::DEFAULT_MULTIPLIER,
    );

    let scale = et.generate_scale();
    let scale_freqs = scale.get_frequencies_vector();

    let mut scale_wave_tables: Vec<wave_table::WaveTable> = Vec::new();

    for f in &scale_freqs {
	let wt = args.voice.get_wavetable(
	    *f,
	    wave_table::DEFAULT_SAMPLE_RATE,
	    wave_table::DEFAULT_AMPLITUDE,
	    wave_table::DEFAULT_PHASE
	);

	scale_wave_tables.push(wt);
    }
    let instrument = instrument::Instrument::new(scale_wave_tables);

    let (swap_tx, swap_rx) = unbounded::<Vec<wave_table::WaveTable>>();

    let state = std::sync::Arc::new(std::sync::Mutex::new(app_state::AppState::new(
	args.voice.clone(),
	app_state::TuningSystemList::EqualTemperment(app_state::EtParams {
	    base_freq: args.base_freq,
	    subdivisions: args.subdivisions,
	    multiplier: equal_temperment::DEFAULT_MULTIPLIER,
	}),
	scale_freqs,
    )));

    let rt = tokio::runtime::Runtime::new().expect("failed to build tokio runtime");
    rt.spawn(async move {
	output::Output::jack_output(args.base_freq, args.subdivisions, instrument, swap_rx).await;
    });
    // TODO: real JACK liveness signal would require an output.rs edit.
    state.lock().unwrap().jack_active = true;

    let (gui_on_tx, gui_on_rx) = unbounded::<u16>();
    let (gui_off_tx, gui_off_rx) = unbounded::<u16>();
    rt.spawn(async move {
	keyboard::create_keyboard_listener(gui_on_tx, gui_off_tx).await.ok();
    });

    gui::run(swap_tx, state, gui_on_rx, gui_off_rx).expect("gui exited with error");
}
