use crossbeam_channel::Sender;
use eframe::egui;

use crate::app_state::SharedState;
use crate::wave_table::WaveTable;

pub struct App {
    _swap_tx: Sender<Vec<WaveTable>>,
    _state: SharedState,
}

impl App {
    pub fn new(swap_tx: Sender<Vec<WaveTable>>, state: SharedState) -> Self {
	Self { _swap_tx: swap_tx, _state: state }
    }
}

impl eframe::App for App {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
}

pub fn run(swap_tx: Sender<Vec<WaveTable>>, state: SharedState) -> eframe::Result<()> {
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
	    Ok(Box::new(App::new(swap_tx, state)))
	}),
    )
}
