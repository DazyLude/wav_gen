use crate::math;
use crate::track;
use crate::track::{Mask, Track};

pub enum InstrumentList {
    None,
    SineWave,
    Xylophone,
}
// Note is a struct that contains data about >>>main<<< frequency of a sound,
// it's length and when it starts. Different instruments will produce different soundwaves.
pub struct Note {
    // Frequency can define note by itself. It's the thing that really matters. In Hz.
    freq: f64,
    // Measured in seconds, time from start of a note 'till it's quiet again. In seconds.
    leng: f64,
    // Time from the start of the track, when this should be played, in seconds.
    time: f64,
    // Relative loudness of the note
    loud: f64,
}

impl Note {
    pub fn zeroed() -> Note {
        Note {
            freq: 0.,
            leng: 0.,
            time: 0.,
            loud: 0.,
        }
    }
    pub fn new(freq: f64, leng: f64, time: f64) -> Note {
        Note {
            freq,
            leng,
            time,
            loud: 1.,
        }
    }

    pub fn next(&self, freq: f64, leng: f64) -> Note {
        Note::new(freq, leng, self.time + self.leng)
    }
    // left here for hystorical reasons
    // that's how notes look without harmonics module
    pub fn default_track() -> Vec<Note> {
        let first: Note = Note::new(440. * (-3.0_f64 / 4.0_f64).exp2(), 1., 0.5);
        let second = first.next(440. * (-1.0_f64 / 3.0_f64).exp2(), 1.);
        let third = second.next(440. * (-1.0_f64 / 6.0_f64).exp2(), 1.);
        let fourth = third.next(440. * (-1.0_f64 / 3.0_f64).exp2(), 1.);
        let fifth = fourth.next(440. * (-3.0_f64 / 4.0_f64).exp2(), 1.);
        let track: Vec<Note> = vec![first, second, third, fourth, fifth];
        track
    }
}

use std::fmt;
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Freq: {};\tLength: {};\tTiming: {};\tLoud: {};",
            self.freq, self.leng, self.time, self.loud
        )
    }
}

// Instruments are compilation of methods and coefficients that turn notes into soundwaves
// Simplest one is a sinewave.

pub trait Instrument {
    fn from_parameters(&mut self, parameters: Vec<&(String, String)>) -> Result<i64, &'static str> {
        let mut successes: i64 = 0;
        for param in parameters {
            match self.update(param) {
                Ok(_) => successes += 1,
                Err(e) => return Err(e),
            }
        }
        return Ok(successes);
    }
    fn track_from_notes(&self, part: &Vec<Note>) -> Track {
        let mut temp_track: Track = Track::new();
        if !part.is_empty() {
            for note in part {
                temp_track = temp_track.mix(&mut self.single_note(note));
            }
        }
        temp_track
    }
    fn new() -> Self;
    fn update(&mut self, param: &(String, String)) -> Result<(), &'static str>;
    fn single_note(&self, note: &Note) -> Track;
}

// Just a sinewave
pub struct SineWave {
    freq_mod: f64,
    volume: f64,
}

impl Instrument for SineWave {
    fn new() -> SineWave {
        SineWave {
            freq_mod: 1.0,
            volume: 1.0,
        }
    }
    fn update(&mut self, param: &(String, String)) -> Result<(), &'static str> {
        match param.0.as_str() {
            "freq_mod" => {
                self.freq_mod = match param.1.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("failed parsing str to f64 in freq_mod"),
                }
            }
            "volume" => {
                self.volume = match param.1.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("failed parsing str to f64 in volume"),
                }
            }
            _ => return Err("setting an unexisting parameter"),
        }
        return Ok(());
    }

    fn single_note(&self, note: &Note) -> Track {
        let mut freq = note.freq * self.freq_mod;
        // this truncates sine a bit so that it ends with 0
        let length = ((note.leng) * 2.0 * freq).trunc() / 2.0 / freq;
        freq *= 2.0 * std::f64::consts::PI;
        let loud = note.loud * self.volume;
        let mut target_vector: Vec<f64> = Vec::new();
        let times = math::linspace_from_n(0., length, Track::time_to_sample_index(length));
        for i in times {
            target_vector.push(loud * (freq * i).sin());
        }
        let mut note_track = Track {
            track: target_vector,
            starting_sample_index: Track::time_to_sample_index(note.time),
            loudness: 1.,
        };
        track::LinearFadeInOut::out_l(length / 100.).apply(&mut note_track);
        track::LinearFadeInOut::in_l(length / 100.).apply(&mut note_track);
        note_track
    }
}

pub struct Xylophone {
    volume: f64,
    clickiness: f64,
    freq_mod: f64,
}

impl Instrument for Xylophone {
    fn new() -> Xylophone {
        Xylophone {
            volume: 1.,
            clickiness: 0.2,
            freq_mod: 1.,
        }
    }
    fn update(&mut self, param: &(String, String)) -> Result<(), &'static str> {
        match param.0.as_str() {
            "freq_mod" => {
                self.freq_mod = match param.1.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("failed parsing str to f64 in freq_mod"),
                }
            }
            "volume" => {
                self.volume = match param.1.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("failed parsing str to f64 in volume"),
                }
            }
            "clickiness" => {
                self.volume = match param.1.parse::<f64>() {
                    Ok(val) => val,
                    Err(_) => return Err("failed parsing str to f64 in clickiness"),
                }
            }
            _ => return Err("setting an unexisting parameter"),
        }
        return Ok(());
    }
    fn single_note(&self, note: &Note) -> Track {
        let mut freq_list: [f64; 101] = [0.; 101];
        for i in 0..101 {
            // i - 50 / 100 is cool
            // just i is a bit curser
            freq_list[i] =
                note.freq * self.freq_mod * 2. * std::f64::consts::PI * (1.02 - i as f64 / 250.);
        }
        let t0 = note.time - self.clickiness;
        let t1 = note.time + self.clickiness;

        let mut target_vector: Vec<f64> = Vec::new();
        let times = math::linspace_from_n(t0, t1, Track::time_to_sample_index(t1 - t0));
        for i in times {
            let mut temp_val = 0.0_f64;
            for freq in freq_list {
                temp_val = temp_val + ((freq * (i - t0)).sin()) as f64;
            }
            target_vector.push(temp_val * note.loud * self.volume);
        }
        Track {
            track: target_vector,
            starting_sample_index: Track::time_to_sample_index(t0),
            loudness: 1.0,
        }
    }
}
