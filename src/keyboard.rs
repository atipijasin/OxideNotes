use nannou::prelude::*;

const START_NOTE: usize = 21; // MIDI A0

pub enum KeyColor {
    Black,
    White,
}

pub fn  init_piano_keyboard(draw: &Draw, window: Rect) {
    let total_white_keys = 52;
    let is_black_key = [false, true, false, true, false, false, true, false, true, false, true, false];

    let mut white_key_to_midi = Vec::with_capacity(total_white_keys);
    let mut white_key_positions = Vec::with_capacity(total_white_keys);
    let mut current_midi_note = START_NOTE;

    for _ in 0..=88 {
        if !is_black_key[current_midi_note % 12] {
            white_key_to_midi.push(current_midi_note);
            white_key_positions.push(-window.w() / 2.0 + white_key_to_midi.len() as f32 * white_key_width(window));
        }
        current_midi_note += 1;
    }

    for (i, _midi_note) in white_key_to_midi.iter().enumerate() {
        let x = -window.w() / 2.0 + (i as f32 + 0.5) * white_key_width(window);
        let y = -window.h() / 2.0 + window.h() * 0.2 / 2.0;

        draw_key(draw, x, y, KeyColor::White, window);
    }

    current_midi_note = START_NOTE;
    while current_midi_note <= 108 {
        if is_black_key[current_midi_note % 12] {
            let prev_white_index = white_key_to_midi.iter().position(|&k| k == current_midi_note - 1);
            if let Some(index) = prev_white_index {
                let x = white_key_positions[index];
                let y = -window.h() * 0.36;

                draw_key(draw, x, y, KeyColor::Black, window);
            }
        }
        current_midi_note += 1;
    }
}

pub fn keyboard_height(window: Rect) -> f32 { window.h() * 0.2 }
fn white_key_width(window: Rect) -> f32 { window.w() / (52.0) }

fn draw_key(draw: &Draw, x: f32, y: f32, color: KeyColor, window: Rect) {
    let (w, h, color, stroke) = match color {
        KeyColor::White => (white_key_width(window), keyboard_height(window), WHITE, BLACK),
        KeyColor::Black => (white_key_width(window) * 0.6, keyboard_height(window) * 0.6, BLACK, DIMGREY),
    };

    draw.rect()
        .xy(pt2(x, y))
        .w_h(w, h)
        .color(color)
        .stroke(stroke)
        .stroke_weight(1.0);
}