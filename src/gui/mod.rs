use crossbeam_channel::{Receiver, Sender};
use eframe::egui;

use crate::app_state::{KEY_GLYPHS, SharedState};
use crate::envelope::EnvelopeParams;
use crate::keyboard::{KEY_EVENT_LEFT_SHIFT, KEY_EVENT_RIGHT_SHIFT};
use crate::wave_table::WaveTable;

mod keyboard_widget;
mod voice_panel;
mod tuning_panel;
mod envelope_panel;
mod jack_indicator;
mod pedal_indicator;
mod info_bar;

pub struct App {
    swap_tx: Sender<Vec<WaveTable>>,
    env_tx: Sender<EnvelopeParams>,
    state: SharedState,
    key_on_rx: Receiver<u16>,
    key_off_rx: Receiver<u16>,
    midi_sustain_rx: Receiver<bool>,
}

impl App {
    pub fn new(
	swap_tx: Sender<Vec<WaveTable>>,
	env_tx: Sender<EnvelopeParams>,
	state: SharedState,
	key_on_rx: Receiver<u16>,
	key_off_rx: Receiver<u16>,
	midi_sustain_rx: Receiver<bool>,
    ) -> Self {
	Self {
	    swap_tx,
	    env_tx,
	    state,
	    key_on_rx,
	    key_off_rx,
	    midi_sustain_rx,
	}
    }

    fn drain_key_events(&mut self) {
	let mut s = self.state.lock().unwrap();
	while let Ok(idx) = self.key_off_rx.try_recv() {
	    match idx {
		KEY_EVENT_LEFT_SHIFT => { s.left_shift_active = false; }
		KEY_EVENT_RIGHT_SHIFT => { s.right_shift_active = false; }
		_ => { s.pressed.remove(&idx); }
	    }
	}
	while let Ok(idx) = self.key_on_rx.try_recv() {
	    match idx {
		KEY_EVENT_LEFT_SHIFT => { s.left_shift_active = true; }
		KEY_EVENT_RIGHT_SHIFT => { s.right_shift_active = true; }
		_ => {
		    s.pressed.insert(idx);
		    let i = idx as usize;
		    let glyph = KEY_GLYPHS.get(i).copied().unwrap_or('?');
		    let freq = s.scale_freqs.get(i).copied().unwrap_or(0.0);
		    s.last_key = Some((glyph, freq));
		}
	    }
	}
	while let Ok(v) = self.midi_sustain_rx.try_recv() {
	    s.midi_sustain_active = v;
	}
	s.sustain_active = s.left_shift_active || s.right_shift_active || s.midi_sustain_active;
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
	self.drain_key_events();
	let mut needs_rebuild = false;
	let ctx = ui.ctx().clone();

	{
	    let s = self.state.lock().unwrap();
	    egui::TopBottomPanel::bottom("info_bar").show(&ctx, |ui| {
		info_bar::show(ui, &s);
	    });
	}

	let mut s = self.state.lock().unwrap();
	ui.horizontal(|ui| {
	    jack_indicator::show(ui, &s);
	    ui.label("jack");
	    ui.add_space(8.0);
	    pedal_indicator::show(ui, &s);
	    ui.label("sustain");
	});
	if voice_panel::show_top_bar(ui, &mut s) {
	    needs_rebuild = true;
	}
	if voice_panel::show_config_window(ui.ctx(), &mut s) {
	    needs_rebuild = true;
	}
	if tuning_panel::show_top_bar(ui, &mut s) {
	    needs_rebuild = true;
	}
	if tuning_panel::show_config_window(ui.ctx(), &mut s) {
	    needs_rebuild = true;
	}
	if let Some(new_env) = envelope_panel::show_top_bar(ui, &mut s) {
	    let _ = self.env_tx.send(new_env);
	}
	ui.separator();
	keyboard_widget::show(ui, &s);
	if needs_rebuild {
	    let new_tables = voice_panel::rebuild_wavetables(&s);
	    let _ = self.swap_tx.send(new_tables);
	}
	ctx.request_repaint();
    }
}

pub fn run(
    swap_tx: Sender<Vec<WaveTable>>,
    env_tx: Sender<EnvelopeParams>,
    state: SharedState,
    key_on_rx: Receiver<u16>,
    key_off_rx: Receiver<u16>,
    midi_sustain_rx: Receiver<bool>,
) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
	viewport: egui::ViewportBuilder::default()
	    .with_inner_size([800.0, 500.0])
	    .with_title("qwertysynth"),
	..Default::default()
    };
    eframe::run_native(
	"qwertysynth",
	options,
	Box::new(|cc| {
	    cc.egui_ctx.set_visuals(egui::Visuals::dark());
	    Ok(Box::new(App::new(swap_tx, env_tx, state, key_on_rx, key_off_rx, midi_sustain_rx)))
	}),
    )
}
