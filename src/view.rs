use nannou::prelude::*;
use crate::keyboard::init_piano_keyboard;
use crate::model::Model;

pub fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    init_piano_keyboard(&draw, app.window_rect());

    draw.to_frame(app, &frame).unwrap();
}