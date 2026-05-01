use eframe::egui;

use crate::app_state::{AppState, TuningSystemList};

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    let tuning_str = match &state.current_tuning {
	TuningSystemList::EqualTemperment(p) => format!(
	    "{} (base {:.2} Hz, {} per {}-tave)",
	    state.current_tuning.name(),
	    p.base_freq,
	    p.subdivisions,
	    p.multiplier,
	),
    };
    let voice_str = format!("voice: {}", voice_name(&state.current_voice));
    let key_str = match state.last_key {
	Some((g, f)) => format!("{}: {:.2}Hz", g, f),
	None => "—".to_string(),
    };
    ui.horizontal(|ui| {
	ui.label(tuning_str);
	ui.separator();
	ui.label(voice_str);
	ui.separator();
	ui.label(key_str);
    });
}

fn voice_name(v: &crate::voice::VoiceList) -> &'static str {
    match v {
	crate::voice::VoiceList::Sine => "sine",
	crate::voice::VoiceList::AdditiveSynth => "additive synth",
    }
}
