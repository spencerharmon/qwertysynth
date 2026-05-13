use eframe::egui;

use crate::app_state::AppState;

/// Physical QWERTY layout: rows of (glyph, scale_index).
/// Inverse of app_state::KEY_GLYPHS.
const ROWS: &[&[(char, u16)]] = &[
    &[('1',3),('2',7),('3',11),('4',15),('5',19),('6',23),('7',27),('8',31),('9',35),('0',39)],
    &[('q',2),('w',6),('e',10),('r',14),('t',18),('y',22),('u',26),('i',30),('o',34),('p',38)],
    &[('a',1),('s',5),('d',9),('f',13),('g',17),('h',21),('j',25),('k',29),('l',33),(';',37)],
    &[('z',0),('x',4),('c',8),('v',12),('b',16),('n',20),('m',24),(',',28),('.',32),('/',36)],
];

const KEY_SIZE: f32 = 36.0;
const KEY_GAP: f32 = 4.0;

/// Per-row left indent in *key units*, measured from the absolute
/// left edge of the keyboard (i.e. the left edge of Shift on row 3).
/// Matches a US ANSI keyboard:
/// - row 0: `` ` `` is 1u, so `1` starts at column 1.0.
/// - row 1: Tab is 1.5u, so `q` starts at column 1.5.
/// - row 2: Caps Lock is 1.75u, so `a` starts at column 1.75.
/// - row 3: Left Shift is 2.25u, so `z` starts at column 2.25. The
///   Shift cell fills that span, so the row's drawn indent is zero.
const ROW_INDENT_U: [f32; 4] = [1.0, 1.5, 1.75, 0.0];

/// Shift key width in key units. Real US ANSI: left shift 2.25u, right
/// shift 2.75u. We render both at 2.25u for visual symmetry. Left
/// shift's right edge then lands exactly where `z` starts, so row 3
/// needs no separate indent.
const SHIFT_WIDTH_U: f32 = 2.25;

/// Width of a single cell that spans `u` key units. A 2.25u-wide
/// Shift cap occupies 2.25 keycap widths plus 1.25 inter-key gaps,
/// the same horizontal space 2.25 normal keys would use.
fn cell_width(u: f32) -> f32 {
    u * KEY_SIZE + (u - 1.0).max(0.0) * KEY_GAP
}

/// Pixel position of the left edge of column `u` on the grid. One
/// column step is one keycap plus one inter-key gap.
fn column_x(u: f32) -> f32 {
    u * (KEY_SIZE + KEY_GAP)
}

fn cell(ui: &mut egui::Ui, width: f32, label: &str, font_size: f32, active: bool) {
    let painter_color_idle = ui.visuals().widgets.inactive.bg_fill;
    let painter_color_active = ui.visuals().selection.bg_fill;
    let text_color = ui.visuals().text_color();
    let stroke_color = ui.visuals().widgets.inactive.bg_stroke.color;

    let (rect, _resp) = ui.allocate_exact_size(
	egui::vec2(width, KEY_SIZE),
	egui::Sense::hover(),
    );
    let fill = if active { painter_color_active } else { painter_color_idle };
    ui.painter().rect_filled(rect, 4.0, fill);
    ui.painter().rect_stroke(
	rect,
	4.0,
	egui::Stroke::new(1.0, stroke_color),
	egui::StrokeKind::Inside,
    );
    ui.painter().text(
	rect.center(),
	egui::Align2::CENTER_CENTER,
	label,
	egui::FontId::monospace(font_size),
	text_color,
    );
}

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    let shift_width = cell_width(SHIFT_WIDTH_U);
    // Bottom row is the widest. Total = column_x(10 + 2.25) for the
    // right shift's left edge, plus the right shift's own width.
    let total_width = column_x(10.0 + SHIFT_WIDTH_U) + shift_width;

    // MIDI sustain indicator above the keyboard, centered on the
    // widest row.
    let midi_width = cell_width(5.0);
    ui.horizontal(|ui| {
	ui.add_space((total_width - midi_width) * 0.5);
	cell(ui, midi_width, "MIDI sustain (CC 64)", 14.0, state.midi_sustain_active);
    });
    ui.add_space(KEY_GAP);

    for (row_i, row) in ROWS.iter().enumerate() {
	ui.horizontal(|ui| {
	    if row_i == 3 {
		cell(ui, shift_width, "Shift", 14.0, state.left_shift_active);
		ui.add_space(KEY_GAP);
	    } else {
		ui.add_space(column_x(ROW_INDENT_U[row_i]));
	    }
	    for (glyph, idx) in *row {
		cell(ui, KEY_SIZE, &glyph.to_string(), 16.0, state.pressed.contains(idx));
		ui.add_space(KEY_GAP);
	    }
	    if row_i == 3 {
		cell(ui, shift_width, "Shift", 14.0, state.right_shift_active);
	    }
	});
	ui.add_space(KEY_GAP);
    }
}
