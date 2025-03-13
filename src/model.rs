use nannou::prelude::*;
use crate::falling_note::FallingNote;
use crate::midi::{read_file, MidiNote};

pub struct Model {
    pub falling_notes: Vec<FallingNote>,
    pub time: f32,
    pub midi_notes: Vec<MidiNote>,
    pub expired_note_ids: Vec<String>,
    pub active_note_indices: Vec<usize>,
}

pub fn model(_: &App) -> Model {
    let midi_notes = match read_file("midi_example.mid") {
        Ok(notes) => {
            println!("Successfully read {} notes", notes.len());
            notes
        }
        Err(e) => {
            eprintln!("Error reading midi file: {}", e);
            vec![]
        },
    };

    Model {
        falling_notes: Vec::new(),
        time: 0.0,
        midi_notes,
        active_note_indices: Vec::new(),
        expired_note_ids: Vec::new(),
    }
}