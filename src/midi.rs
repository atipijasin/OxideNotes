use std::fs::File;
use std::io::Read;
use std::path::Path;
use midly::{Smf, Timing};

pub struct MidiNote {
    pub note_number: u8,
    pub velocity: u8,
    pub start_time: f64,
    pub duration: f64,
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<MidiNote>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let smf = Smf::parse(&buffer)?;

    let ticks_per_second = match smf.header.timing {
        Timing::Metrical(ticks_per_beat) => {
            let ticks_per_beat = ticks_per_beat.as_int() as f64;
            let beats_per_minute = 120.0; // currently assuming a bpm of 120
            (ticks_per_beat * beats_per_minute) / 60.0;
        }
        Timing::Timecode(fps, subframes) => {
            fps.as_f32() as f64 * subframes as f64;
        }
    };
    Ok(Vec::new())
}