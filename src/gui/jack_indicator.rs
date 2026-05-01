use eframe::egui;

use crate::app_state::AppState;

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    let radius = 7.0;
    let (rect, _) = ui.allocate_exact_size(
	egui::vec2(radius * 2.0 + 2.0, radius * 2.0 + 2.0),
	egui::Sense::hover(),
    );
    let color = if state.jack_active {
	egui::Color32::from_rgb(60, 200, 80)
    } else {
	egui::Color32::from_rgb(80, 80, 80)
    };
    ui.painter().circle_filled(rect.center(), radius, color);
}
