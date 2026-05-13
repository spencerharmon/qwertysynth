use eframe::egui;

use crate::app_state::{AppState, VoiceParams};
use crate::voice::VoiceList;
use crate::voice::additive_synth::{DEFAULT_AMPLITUDES, NUM_PARTIALS};
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
		if ui.button("sawtooth").clicked() {
		    state.current_voice = VoiceList::Sawtooth;
		}
		if ui.button("square").clicked() {
		    state.current_voice = VoiceList::Square;
		}
		if ui.button("triangle").clicked() {
		    state.current_voice = VoiceList::Triangle;
		}
		if ui.button("pwm").clicked() {
		    state.current_voice = VoiceList::Pwm;
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
    let voice = state.current_voice.clone();
    egui::Window::new(title)
	.id(egui::Id::new(voice_window_id(&voice)))
	.open(&mut open)
	.show(ctx, |ui| {
	    let p: &mut VoiceParams = &mut state.voice_params;
	    let r1 = ui.add(egui::Slider::new(&mut p.amplitude, 0.0..=1.0).text("amplitude"));
	    let r2 = ui.add(egui::Slider::new(&mut p.phase, 0..=255).text("phase"));
	    if r1.changed() || r2.changed() {
		changed = true;
	    }
	    if matches!(voice, VoiceList::AdditiveSynth) {
		ui.separator();
		changed |= show_partials(ui, p);
	    }
	    if matches!(voice, VoiceList::Pwm) {
		ui.separator();
		let r = ui.add(egui::Slider::new(&mut p.pwm_duty, 0.05..=0.95).text("duty"));
		if r.changed() {
		    changed = true;
		}
	    }
	});
    state.show_voice_config = open;
    changed
}

fn show_partials(ui: &mut egui::Ui, p: &mut VoiceParams) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
	ui.label("partials");
	if ui.button("reset").clicked() {
	    p.partial_amplitudes = DEFAULT_AMPLITUDES;
	    p.partial_phases = [0.0; NUM_PARTIALS];
	    changed = true;
	}
    });
    egui::Grid::new("partials_grid")
	.num_columns(3)
	.spacing([8.0, 4.0])
	.striped(true)
	.show(ui, |ui| {
	    ui.label("n");
	    ui.label("amplitude");
	    ui.label("phase (rad)");
	    ui.end_row();
	    let two_pi = 2.0 * std::f32::consts::PI;
	    for n in 0..NUM_PARTIALS {
		ui.label(format!("{}", n + 1));
		let r_a = ui.add(
		    egui::Slider::new(&mut p.partial_amplitudes[n], 0.0..=1.0)
			.show_value(true),
		);
		let r_p = ui.add(
		    egui::Slider::new(&mut p.partial_phases[n], 0.0..=two_pi)
			.show_value(true),
		);
		if r_a.changed() || r_p.changed() {
		    changed = true;
		}
		ui.end_row();
	    }
	});
    changed
}

pub fn rebuild_wavetables(state: &AppState) -> Vec<WaveTable> {
    state
	.scale_freqs
	.iter()
	.map(|f| {
	    state.current_voice.get_wavetable_with_params(
		*f,
		DEFAULT_SAMPLE_RATE,
		&state.voice_params,
	    )
	})
	.collect()
}

fn voice_name(v: &VoiceList) -> &'static str {
    match v {
	VoiceList::Sine => "sine",
	VoiceList::AdditiveSynth => "additive synth",
	VoiceList::Sawtooth => "sawtooth",
	VoiceList::Square => "square",
	VoiceList::Triangle => "triangle",
	VoiceList::Pwm => "pwm",
    }
}

fn voice_window_id(v: &VoiceList) -> &'static str {
    match v {
	VoiceList::Sine => "sine_config_window",
	VoiceList::AdditiveSynth => "additive_synth_config_window",
	VoiceList::Sawtooth => "sawtooth_config_window",
	VoiceList::Square => "square_config_window",
	VoiceList::Triangle => "triangle_config_window",
	VoiceList::Pwm => "pwm_config_window",
    }
}
