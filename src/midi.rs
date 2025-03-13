use midly::{MidiMessage, Smf, Timing, TrackEventKind};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct MidiNote {
    pub note_number: u8,
    pub start_time: f64,
    pub duration: f64,
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<MidiNote>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let smf = Smf::parse(&buffer)?; // standard midi file

    let ticks_per_second = match smf.header.timing {
        Timing::Metrical(ticks_per_beat) => {
            let ticks_per_beat = ticks_per_beat.as_int() as f64;
            let beats_per_minute = 120.0; // currently assuming a bpm of 120
            (ticks_per_beat * beats_per_minute) / 60.0
        }
        Timing::Timecode(fps, subframes) => {
            fps.as_f32() as f64 * subframes as f64
        }
    };

    let mut notes = Vec::new();
    let mut active_notes = HashMap::new();
    let mut current_time_ticks: u64 = 0;

    for track in smf.tracks {
        current_time_ticks = 0;
        for event in track {
            current_time_ticks += event.delta.as_int() as u64;
            let current_time_seconds = current_time_ticks as f64 / ticks_per_second;

            if let TrackEventKind::Midi { channel, message } = event.kind {
                match message {
                    MidiMessage::NoteOn { key, vel } if vel.as_int() > 0 => {
                        active_notes.insert((channel.as_int(), key.as_int()), current_time_seconds);
                    }
                    MidiMessage::NoteOff { key, vel } | MidiMessage::NoteOn { key, vel } => {
                        if let Some(start_time) = active_notes.remove(&(channel.as_int(), key.as_int()))
                        {
                            notes.push(MidiNote {
                                note_number: key.as_int(),
                                start_time,
                                duration: current_time_seconds - start_time,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(notes)
}
