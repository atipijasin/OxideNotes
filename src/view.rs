use nannou::prelude::*;
use crate::falling_note::draw_falling_notes;
use crate::keyboard::init_piano_keyboard;
use crate::model::Model;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    init_piano_keyboard(&draw, app.window_rect());
    draw_falling_notes(&draw, model);

    let timer_text = format!("{:.1}s", model.time);
    draw.text(&timer_text)
        .font_size(24)
        .color(WHITE)
        .xy(Vec2::new(
            app.window_rect().right() - 50.0,
            app.window_rect().top() - 30.0
        ));

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}