use eframe::egui;

use crate::app_state::{AppState, EtParams, TuningSystemList};
use crate::equal_temperment::EqualTemperment;

/// Returns true if the scale should be regenerated and the wavetable
/// set rebuilt + pushed.
pub fn show_top_bar(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    ui.horizontal(|ui| {
	ui.label("tuning:");
	egui::ComboBox::from_id_salt("tuning_combo")
	    .selected_text(state.current_tuning.name())
	    .show_ui(ui, |ui| {
		if ui.button("equal temperment").clicked() {
		    if !matches!(state.current_tuning, TuningSystemList::EqualTemperment(_)) {
			state.current_tuning = TuningSystemList::EqualTemperment(EtParams {
			    base_freq: 440.0,
			    subdivisions: 33,
			    multiplier: 2,
			});
		    }
		}
	    });
	if ui.button("configure tuning…").clicked() {
	    state.show_tuning_config = !state.show_tuning_config;
	}
    });
    false
}

pub fn show_config_window(ctx: &egui::Context, state: &mut AppState) -> bool {
    if !state.show_tuning_config {
	return false;
    }
    let mut applied = false;
    let mut open = state.show_tuning_config;
    egui::Window::new("equal temperment config")
	.id(egui::Id::new("et_config_window"))
	.open(&mut open)
	.show(ctx, |ui| {
	    let TuningSystemList::EqualTemperment(p) = &mut state.current_tuning;
	    ui.add(egui::Slider::new(&mut p.base_freq, 20.0..=2000.0).text("base freq (Hz)"));
	    ui.add(egui::Slider::new(&mut p.subdivisions, 1..=64).text("subdivisions"));
	    ui.add(egui::Slider::new(&mut p.multiplier, 2..=8).text("multiplier"));
	    if ui.button("apply").clicked() {
		applied = true;
	    }
	});
    state.show_tuning_config = open;
    if applied {
	regenerate_scale(state);
    }
    applied
}

fn regenerate_scale(state: &mut AppState) {
    let TuningSystemList::EqualTemperment(p) = &state.current_tuning;
    let et = EqualTemperment::new(p.base_freq, p.subdivisions, p.multiplier);
    state.scale_freqs = et.generate_scale().get_frequencies_vector();
}
