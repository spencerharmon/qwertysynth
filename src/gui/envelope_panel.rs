use eframe::egui;

use crate::app_state::AppState;
use crate::envelope::EnvelopeParams;

/// Returns Some(new_params) if any slider moved this frame.
pub fn show_top_bar(ui: &mut egui::Ui, state: &mut AppState) -> Option<EnvelopeParams> {
    let mut changed = false;
    ui.horizontal(|ui| {
	ui.label("envelope:");
	let p = &mut state.envelope_params;
	let r1 = ui.add(
	    egui::Slider::new(&mut p.attack_s, 0.001..=2.0)
		.logarithmic(true)
		.text("A")
		.suffix(" s"),
	);
	let r2 = ui.add(
	    egui::Slider::new(&mut p.decay_s, 0.001..=2.0)
		.logarithmic(true)
		.text("D")
		.suffix(" s"),
	);
	let r3 = ui.add(
	    egui::Slider::new(&mut p.sustain, 0.0..=1.0).text("S"),
	);
	let r4 = ui.add(
	    egui::Slider::new(&mut p.release_s, 0.001..=2.0)
		.logarithmic(true)
		.text("R")
		.suffix(" s"),
	);
	if r1.changed() || r2.changed() || r3.changed() || r4.changed() {
	    changed = true;
	}
    });
    if changed { Some(state.envelope_params) } else { None }
}
