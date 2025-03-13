use crate::falling_note::FallingNote;
use crate::model::Model;
use nannou::App;
use nannou::event::Update;
use nannou::geom::pt2;
use nannou::math::map_range;
use crate::keyboard::keyboard_height;

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

        if should_show_note(note.start_time, model.time as f64, &model.falling_notes, &model.expired_note_ids, note.note_number)
        {
            model.falling_notes.push(FallingNote {
                // TODO refactor id extraction
                id: note.note_number.to_string() + " " + note.start_time.to_string().as_str(),
                position: pt2(x, app.window_rect().top()),
                width: 20.0,
                height: 80.0 * (note.duration as f32).min(3.0),
                velocity: 2.0,
            });
        }

        // model.active_note_indices.push(i);
    }

    for note in &mut model.falling_notes {
        note.position.y -= note.velocity;
    }


    model.falling_notes.retain(|note|  {
        let result = note.position.y - (note.height * 0.5) > -app.window_rect().h() * 0.5 + keyboard_height(app.window_rect());
        if !result { model.expired_note_ids.push(note.id.clone()); }
        return result
    })
}

fn should_show_note(start_time: f64, current_time: f64, current_fallng_notes: &Vec<FallingNote>, current_expired_note_ids: &Vec<String>, note_number: u8) -> bool {
    // TODO refactor this mess..
    start_time < current_time && !current_fallng_notes.iter().any(|n| { n.id == note_number.to_string() + " " + start_time.to_string().as_str() })  && !current_expired_note_ids.iter().any(|note_id| note_id.clone() == (note_number.to_string() + " " + start_time.to_string().as_str()))
}
