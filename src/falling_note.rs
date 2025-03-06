use nannou::color::ORANGE;
use crate::model::Model;
use nannou::Draw;
use nannou::geom::Point2;

pub struct FallingNote {
    pub position: Point2,
    pub width: f32,
    pub height: f32,
    pub velocity: f32,
}

pub fn draw_falling_notes(draw: &Draw, model: &Model) {
    for note in model.falling_notes.iter() {
        draw.rect()
            .xy(note.position)
            .w_h(note.width, note.height)
            .color(ORANGE);
    }
}
