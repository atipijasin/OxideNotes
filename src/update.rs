use nannou::App;
use nannou::event::Update;
use crate::model::Model;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    for note in &mut model.falling_notes {
        note.position.y -= note.velocity;

        if note.position.y < app.window_rect().bottom() - note.height {
            note.position.y = app.window_rect().top() + note.height;
        }
    }
}