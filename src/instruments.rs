use crate::math;
use crate::track;

// Note is a struct that contains data about >>>main<<< frequency of a sound,
// it's length and when it starts. Different instruments will produce different soundwaves.

pub struct Note {
    // Frequency can define note by itself. It's the thing that really matters. In Hz.
    freq: f64,
    // Measured in seconds, time from start of a note 'till it's quiet again. In seconds.
    leng: f64,
    // Time from the start of the track, when this should be played, in seconds.
    time: f64,
}

impl Note {
    pub fn new(freq: f64, leng: f64, time: f64) -> Note {
        Note { freq, leng, time }
    }

    pub fn next(&self, freq: f64, leng: f64) -> Note {
        Note::new(freq, leng, self.time + self.leng)
    }
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

// Instruments are compilation of methods and coefficients that turn notes into soundwaves
// Simplest one is a sinewave.

pub trait Instrument {
    fn track_from_notes(part: &Vec<Note>) -> track::Track;
}

// Just a sinewave

pub struct SineWave {}

impl SineWave {
    pub fn gen_sine_wave_from_x0(freq: f64, t0: f64, t1: f64, x0: f64, deriv: f64) -> Vec<f64> {
        assert!(t0 < t1, "t0 ({t0}) should be less then t1 ({t1})");
        let freq_r = freq * 2. * std::f64::consts::PI;
        let phase_sign = (x0 - deriv).signum();
        let phase_shift = if phase_sign > 0. {
            (x0).asin()
        } else {
            std::f64::consts::PI - (x0).asin()
        };
        let mut target_vector: Vec<f64> = Vec::new();
        let times =
            math::linspace_from_n(t0, t1, (t1 - t0) as i64 * track::DESIRED_SAMPLE_RATE as i64);
        for i in times {
            target_vector.push((phase_shift + freq_r * (i - t0)).sin());
        }
        target_vector
    }
}

impl Instrument for SineWave {
    fn track_from_notes(part: &Vec<Note>) -> track::Track {
        let mut track: Vec<f64> = Vec::new();
        if !part.is_empty() {
            let mut x0 = 0.;
            let mut x_deriv = 0.;
            for note in part {
                track.append(&mut SineWave::gen_sine_wave_from_x0(
                    note.freq,
                    note.time,
                    note.leng + note.time,
                    x0,
                    x_deriv,
                ));
                x0 = track.pop().unwrap_or(0.);
                x_deriv = *track.last().unwrap_or(&0.);
            }
        }
        track::Track::from(track)
    }
}
