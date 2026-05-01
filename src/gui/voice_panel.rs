use eframe::egui;

use crate::app_state::{AppState, VoiceParams};
use crate::voice::VoiceList;
use crate::wave_table::{WaveTable, DEFAULT_SAMPLE_RATE};

/// Returns true if the wavetable set should be rebuilt and pushed.
pub fn show_top_bar(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
	ui.label("voice:");
	let prev = std::mem::discriminant(&state.current_voice);
	egui::ComboBox::from_id_salt("voice_combo")
	    .selected_text(voice_name(&state.current_voice))
	    .show_ui(ui, |ui| {
		if ui.button("sine").clicked() {
		    state.current_voice = VoiceList::Sine;
		}
		if ui.button("additive synth").clicked() {
		    state.current_voice = VoiceList::AdditiveSynth;
		}
	    });
	if std::mem::discriminant(&state.current_voice) != prev {
	    changed = true;
	}
	if ui.button("configure voice…").clicked() {
	    state.show_voice_config = !state.show_voice_config;
	}
    });
    changed
}

pub fn show_config_window(ctx: &egui::Context, state: &mut AppState) -> bool {
    if !state.show_voice_config {
	return false;
    }
    let mut changed = false;
    let mut open = state.show_voice_config;
    let title = format!("{} config", voice_name(&state.current_voice));
    egui::Window::new(title)
	.id(egui::Id::new(voice_window_id(&state.current_voice)))
	.open(&mut open)
	.show(ctx, |ui| {
	    let p: &mut VoiceParams = &mut state.voice_params;
	    let r1 = ui.add(egui::Slider::new(&mut p.amplitude, 0.0..=1.0).text("amplitude"));
	    let r2 = ui.add(egui::Slider::new(&mut p.phase, 0..=255).text("phase"));
	    if r1.changed() || r2.changed() {
		changed = true;
	    }
	});
    state.show_voice_config = open;
    changed
}

pub fn rebuild_wavetables(state: &AppState) -> Vec<WaveTable> {
    state
	.scale_freqs
	.iter()
	.map(|f| {
	    state.current_voice.get_wavetable(
		*f,
		DEFAULT_SAMPLE_RATE,
		state.voice_params.amplitude,
		state.voice_params.phase,
	    )
	})
	.collect()
}

fn voice_name(v: &VoiceList) -> &'static str {
    match v {
	VoiceList::Sine => "sine",
	VoiceList::AdditiveSynth => "additive synth",
    }
}

fn voice_window_id(v: &VoiceList) -> &'static str {
    match v {
	VoiceList::Sine => "sine_config_window",
	VoiceList::AdditiveSynth => "additive_synth_config_window",
    }
}
