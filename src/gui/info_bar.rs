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
	TuningSystemList::Harmonic(p) => format!(
	    "{} (fundamental {:.2} Hz, start {})",
	    state.current_tuning.name(),
	    p.fundamental,
	    p.start_harmonic,
	),
	TuningSystemList::Mos(p) => format!(
	    "{} (base {:.2} Hz, gen {:.4}, frame {:.3}, size {})",
	    state.current_tuning.name(),
	    p.base_freq,
	    p.generator,
	    p.framing_interval,
	    p.mos_size,
	),
	TuningSystemList::Lattice(p) => format!(
	    "{} (base {:.2} Hz, 3-limit ±{}, 5-limit ±{})",
	    state.current_tuning.name(),
	    p.base_freq,
	    p.three_limit,
	    p.five_limit,
	),
	TuningSystemList::SternBrocot(p) => format!(
	    "{} (base {:.2} Hz, frame {:.3})",
	    state.current_tuning.name(),
	    p.base_freq,
	    p.framing_interval,
	),
	TuningSystemList::Scala(p) => {
	    let desc = p.loaded.as_ref().map(|f| f.description.as_str()).unwrap_or("(no file)");
	    format!(
		"{} (base {:.2} Hz, {})",
		state.current_tuning.name(),
		p.base_freq,
		desc,
	    )
	}
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
