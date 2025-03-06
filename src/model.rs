use nannou::prelude::*;
use crate::falling_note::FallingNote;

pub struct Model {
    pub falling_notes: Vec<FallingNote>,
}

pub fn model(app: &App) -> Model {
    Model {
        falling_notes: vec![FallingNote {
            position: pt2(0.0, app.window_rect().top()),
            width: 20.0,
            height: 80.0,
            velocity: 2.0, // pixels per frame
        }]
    }
}