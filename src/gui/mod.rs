use crossbeam_channel::{Receiver, Sender};
use eframe::egui;

use crate::app_state::{KEY_GLYPHS, SharedState};
use crate::wave_table::WaveTable;

mod keyboard_widget;
mod voice_panel;

pub struct App {
    swap_tx: Sender<Vec<WaveTable>>,
    state: SharedState,
    key_on_rx: Receiver<u16>,
    key_off_rx: Receiver<u16>,
}

impl App {
    pub fn new(
	swap_tx: Sender<Vec<WaveTable>>,
	state: SharedState,
	key_on_rx: Receiver<u16>,
	key_off_rx: Receiver<u16>,
    ) -> Self {
	Self { swap_tx, state, key_on_rx, key_off_rx }
    }

    fn drain_key_events(&self) {
	let mut s = self.state.lock().unwrap();
	while let Ok(idx) = self.key_off_rx.try_recv() {
	    s.pressed.remove(&idx);
	}
	while let Ok(idx) = self.key_on_rx.try_recv() {
	    s.pressed.insert(idx);
	    let i = idx as usize;
	    let glyph = KEY_GLYPHS.get(i).copied().unwrap_or('?');
	    let freq = s.scale_freqs.get(i).copied().unwrap_or(0.0);
	    s.last_key = Some((glyph, freq));
	}
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
	self.drain_key_events();
	let mut needs_rebuild = false;
	{
	    let mut s = self.state.lock().unwrap();
	    if voice_panel::show_top_bar(ui, &mut s) {
		needs_rebuild = true;
	    }
	    if voice_panel::show_config_window(ui.ctx(), &mut s) {
		needs_rebuild = true;
	    }
	    ui.separator();
	    keyboard_widget::show(ui, &s);
	    if needs_rebuild {
		let new_tables = voice_panel::rebuild_wavetables(&s);
		let _ = self.swap_tx.send(new_tables);
	    }
	}
	ui.ctx().request_repaint();
    }
}

pub fn run(
    swap_tx: Sender<Vec<WaveTable>>,
    state: SharedState,
    key_on_rx: Receiver<u16>,
    key_off_rx: Receiver<u16>,
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
	    Ok(Box::new(App::new(swap_tx, state, key_on_rx, key_off_rx)))
	}),
    )
}
