use eframe::egui;

use crate::app_state::AppState;

/// Physical QWERTY layout: rows of (glyph, scale_index).
/// Inverse of app_state::KEY_GLYPHS.
const ROWS: &[&[(char, u16)]] = &[
    &[('`',0),('1',3),('2',7),('3',11),('4',15),('5',19),('6',23),('7',27),('8',31),('9',35),('0',39)],
    &[('q',2),('w',6),('e',10),('r',14),('t',18),('y',22),('u',26),('i',30),('o',34),('p',38)],
    &[('a',1),('s',5),('d',9),('f',13),('g',17),('h',21),('j',25),('k',29),('l',33),(';',37)],
    &[('z',4),('x',8),('c',12),('v',16),('b',20),('n',24),('m',28),(',',32),('.',36)],
];

const KEY_SIZE: f32 = 36.0;
const KEY_GAP: f32 = 4.0;
const ROW_INDENT: [f32; 4] = [0.0, 18.0, 27.0, 36.0];

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    let painter_color_idle = ui.visuals().widgets.inactive.bg_fill;
    let painter_color_active = ui.visuals().selection.bg_fill;
    let text_color = ui.visuals().text_color();

    for (row_i, row) in ROWS.iter().enumerate() {
	ui.horizontal(|ui| {
	    ui.add_space(ROW_INDENT[row_i]);
	    for (glyph, idx) in *row {
		let (rect, _resp) = ui.allocate_exact_size(
		    egui::vec2(KEY_SIZE, KEY_SIZE),
		    egui::Sense::hover(),
		);
		let pressed = state.pressed.contains(idx);
		let fill = if pressed { painter_color_active } else { painter_color_idle };
		ui.painter().rect_filled(rect, 4.0, fill);
		ui.painter().rect_stroke(
		    rect,
		    4.0,
		    egui::Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color),
		    egui::StrokeKind::Inside,
		);
		ui.painter().text(
		    rect.center(),
		    egui::Align2::CENTER_CENTER,
		    glyph.to_string(),
		    egui::FontId::monospace(16.0),
		    text_color,
		);
		ui.add_space(KEY_GAP);
	    }
	});
	ui.add_space(KEY_GAP);
    }
}
