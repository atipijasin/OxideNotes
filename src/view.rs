use nannou::prelude::*;
use crate::falling_note::draw_falling_notes;
use crate::keyboard::init_piano_keyboard;
use crate::model::Model;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    init_piano_keyboard(&draw, app.window_rect());
    draw_falling_notes(&draw, model);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}