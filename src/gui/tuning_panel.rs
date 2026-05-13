use eframe::egui;

use crate::app_state::{
    AppState, EtParams, HarmonicParams, LatticeParams, MosParams, ScalaParams,
    SternBrocotParams, TuningSystemList,
};
use crate::equal_temperment::EqualTemperment;
use crate::tuning::TuningSystem;
use crate::tuning::harmonic::HarmonicSeries;
use crate::tuning::lattice::LatticeScale;
use crate::tuning::mos::MosScale;
use crate::tuning::scala::{self, ScalaScale};
use crate::tuning::stern_brocot::SternBrocotScale;

const SCALE_LEN: usize = 40;

/// Returns true if the scale should be regenerated and the wavetable
/// set rebuilt + pushed.
pub fn show_top_bar(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
	ui.label("tuning:");
	egui::ComboBox::from_id_salt("tuning_combo")
	    .selected_text(state.current_tuning.name())
	    .show_ui(ui, |ui| {
		if ui.button("equal temperment").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::EqualTemperment(_))
		{
		    state.current_tuning = TuningSystemList::EqualTemperment(EtParams {
			base_freq: 440.0,
			subdivisions: 33,
			multiplier: 2,
		    });
		    changed = true;
		}
		if ui.button("harmonic series").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::Harmonic(_))
		{
		    state.current_tuning = TuningSystemList::Harmonic(HarmonicParams {
			fundamental: 110.0,
			start_harmonic: 8,
		    });
		    changed = true;
		}
		if ui.button("MOS / generator").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::Mos(_))
		{
		    state.current_tuning = TuningSystemList::Mos(MosParams {
			base_freq: 220.0,
			generator: 1.5,
			framing_interval: 2.0,
			mos_size: 7,
		    });
		    changed = true;
		}
		if ui.button("5-limit lattice").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::Lattice(_))
		{
		    state.current_tuning = TuningSystemList::Lattice(LatticeParams {
			base_freq: 220.0,
			three_limit: 2,
			five_limit: 1,
		    });
		    changed = true;
		}
		if ui.button("Stern-Brocot").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::SternBrocot(_))
		{
		    state.current_tuning = TuningSystemList::SternBrocot(SternBrocotParams {
			base_freq: 220.0,
			framing_interval: 2.0,
		    });
		    changed = true;
		}
		if ui.button("Scala (.scl)").clicked()
		    && !matches!(state.current_tuning, TuningSystemList::Scala(_))
		{
		    state.current_tuning = TuningSystemList::Scala(ScalaParams::default());
		    changed = true;
		}
	    });
	if ui.button("configure tuning…").clicked() {
	    state.show_tuning_config = !state.show_tuning_config;
	}
    });
    if changed {
	regenerate_scale(state);
    }
    changed
}

pub fn show_config_window(ctx: &egui::Context, state: &mut AppState) -> bool {
    if !state.show_tuning_config {
	return false;
    }
    let mut applied = false;
    let mut open = state.show_tuning_config;
    let title = format!("{} config", state.current_tuning.name());
    let window_id = window_id(&state.current_tuning);
    egui::Window::new(title)
	.id(egui::Id::new(window_id))
	.open(&mut open)
	.show(ctx, |ui| {
	    match &mut state.current_tuning {
		TuningSystemList::EqualTemperment(p) => {
		    ui.add(egui::Slider::new(&mut p.base_freq, 20.0..=2000.0).text("base freq (Hz)"));
		    ui.add(egui::Slider::new(&mut p.subdivisions, 1..=64).text("subdivisions"));
		    ui.add(egui::Slider::new(&mut p.multiplier, 2..=8).text("multiplier"));
		}
		TuningSystemList::Harmonic(p) => {
		    ui.add(
			egui::Slider::new(&mut p.fundamental, 20.0..=2000.0)
			    .logarithmic(true)
			    .text("fundamental (Hz)"),
		    );
		    ui.add(egui::Slider::new(&mut p.start_harmonic, 1..=64).text("start harmonic"));
		}
		TuningSystemList::Mos(p) => {
		    ui.add(
			egui::Slider::new(&mut p.base_freq, 20.0..=2000.0)
			    .logarithmic(true)
			    .text("base freq (Hz)"),
		    );
		    ui.add(
			egui::Slider::new(&mut p.generator, 1.001..=3.999)
			    .logarithmic(true)
			    .text("generator"),
		    );
		    ui.add(
			egui::Slider::new(&mut p.framing_interval, 2.0..=8.0)
			    .text("framing interval"),
		    );
		    ui.add(egui::Slider::new(&mut p.mos_size, 2..=31).text("MOS size"));
		}
		TuningSystemList::Lattice(p) => {
		    ui.add(
			egui::Slider::new(&mut p.base_freq, 20.0..=2000.0)
			    .logarithmic(true)
			    .text("base freq (Hz)"),
		    );
		    ui.add(egui::Slider::new(&mut p.three_limit, 0..=6).text("3-limit window (±)"));
		    ui.add(egui::Slider::new(&mut p.five_limit, 0..=4).text("5-limit window (±)"));
		}
		TuningSystemList::SternBrocot(p) => {
		    ui.add(
			egui::Slider::new(&mut p.base_freq, 20.0..=2000.0)
			    .logarithmic(true)
			    .text("base freq (Hz)"),
		    );
		    ui.add(
			egui::Slider::new(&mut p.framing_interval, 2.0..=8.0)
			    .text("framing interval"),
		    );
		}
		TuningSystemList::Scala(p) => {
		    ui.add(
			egui::Slider::new(&mut p.base_freq, 20.0..=2000.0)
			    .logarithmic(true)
			    .text("base freq (Hz)"),
		    );
		    ui.horizontal(|ui| {
			ui.label("file:");
			ui.text_edit_singleline(&mut p.path);
		    });
		    if ui.button("load").clicked() {
			match std::fs::read_to_string(&p.path) {
			    Ok(text) => match scala::parse(&text) {
				Ok(file) => {
				    p.loaded = Some(file);
				    p.last_error = None;
				}
				Err(e) => {
				    p.last_error = Some(format!("parse error: {e}"));
				}
			    },
			    Err(e) => {
				p.last_error = Some(format!("read error: {e}"));
			    }
			}
		    }
		    if let Some(file) = &p.loaded {
			ui.label(format!("loaded: {} ({} ratios)", file.description, file.ratios.len()));
		    }
		    if let Some(err) = &p.last_error {
			ui.colored_label(egui::Color32::from_rgb(220, 80, 80), err);
		    }
		}
	    }
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

fn window_id(t: &TuningSystemList) -> &'static str {
    match t {
	TuningSystemList::EqualTemperment(_) => "et_config_window",
	TuningSystemList::Harmonic(_) => "harmonic_config_window",
	TuningSystemList::Mos(_) => "mos_config_window",
	TuningSystemList::Lattice(_) => "lattice_config_window",
	TuningSystemList::SternBrocot(_) => "stern_brocot_config_window",
	TuningSystemList::Scala(_) => "scala_config_window",
    }
}

fn regenerate_scale(state: &mut AppState) {
    let system: Option<Box<dyn TuningSystem>> = match &state.current_tuning {
	TuningSystemList::EqualTemperment(p) => Some(Box::new(EqualTemperment::new(
	    p.base_freq,
	    p.subdivisions,
	    p.multiplier,
	))),
	TuningSystemList::Harmonic(p) => Some(Box::new(HarmonicSeries::new(
	    p.fundamental,
	    p.start_harmonic,
	))),
	TuningSystemList::Mos(p) => Some(Box::new(MosScale::new(
	    p.base_freq,
	    p.generator,
	    p.framing_interval,
	    p.mos_size,
	))),
	TuningSystemList::Lattice(p) => Some(Box::new(LatticeScale::new(
	    p.base_freq,
	    p.three_limit,
	    p.five_limit,
	))),
	TuningSystemList::SternBrocot(p) => Some(Box::new(SternBrocotScale::new(
	    p.base_freq,
	    p.framing_interval,
	))),
	TuningSystemList::Scala(p) => p.loaded.as_ref().map(|f| {
	    Box::new(ScalaScale::new(p.base_freq, f.clone())) as Box<dyn TuningSystem>
	}),
    };
    if let Some(system) = system {
	state.scale_freqs = system.generate_scale(SCALE_LEN);
    }
    // Scala with no file loaded: keep the previous scale_freqs in
    // place rather than blanking the keyboard. Apply does nothing
    // until the user successfully loads a file.
}
