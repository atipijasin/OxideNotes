use crate::falling_note::FallingNote;
use crate::model::Model;
use nannou::App;
use nannou::event::Update;
use nannou::geom::pt2;
use nannou::math::map_range;

pub fn update(app: &App, model: &mut Model, update: Update) {
    model.time += update.since_last.as_secs_f32();
    for (i, note) in model.midi_notes.iter().enumerate() {
        let x = map_range(
            note.note_number as f32,
            21.0,
            108.0,
            -app.window_rect().w() * 0.5,
            app.window_rect().w() * 0.5,
        );

        if (note.start_time as f32) < model.time
            && !model.falling_notes.iter().any(|n| {
                n.id == note.note_number.to_string() + " " + note.start_time.to_string().as_str()
            })
        {
            model.falling_notes.push(FallingNote {
                id: note.note_number.to_string() + " " + note.start_time.to_string().as_str(),
                position: pt2(x, app.window_rect().top()),
                width: 20.0,
                height: 80.0 * (note.duration as f32).min(3.0),
                velocity: 10.0,
            });
        }

        // model.active_note_indices.push(i);
    }

    for note in &mut model.falling_notes {
        note.position.y -= note.velocity;
    }

    model.falling_notes.retain(|note| note.position.y + note.height * 0.5 > -app.window_rect().h() * 0.5);
}
