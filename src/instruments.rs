use crate::math;

pub const DESIRED_SAMPLE_RATE: u32 = 16000;

// Melody is a sorted vector of tuples, in which first value represents a time of a sample, and second - it's amplitude.
// It is assumed to be sorted by an increasing time, and that there are no duplicates
// Having duplicates will make the last one define the beahaviour of this sample
// Being not sorted will make some methods produce unexpected results, most likely resulting in a lower quality of sound
// It does sound like a good idea use a hashed map instead, but maybe some other day :)

pub struct Melody {
    pub melody: Vec<(f64, f64)>,
}

impl From<Vec<(f64, f64)>> for Melody {
    fn from(thing: Vec<(f64, f64)>) -> Melody {
        Melody { melody: thing }
    }
}

impl From<Melody> for Vec<(f64, f64)> {
    fn from(thing: Melody) -> Vec<(f64, f64)> {
        thing.melody
    }
}

pub enum FadeMode {
    Linear,
}

impl Melody {
    // makes everything to be within [-1; 1] range
    pub fn normalize(&mut self) {
        let mut max_amp: f64 = 0.;
        for sample in &self.melody {
            if max_amp < sample.1.abs() {
                max_amp = sample.1.abs();
            }
        }
        for i in 0..self.melody.len() {
            self.melody[i].1 = self.melody[i].1 / max_amp;
        }
    }
    // simply checks the sortedness of the melody
    pub fn is_sorted(&self) -> bool {
        let mut last_one = -1.;
        for sample in &self.melody {
            if last_one < sample.0 {
                last_one = sample.0;
            } else {
                return false;
            };
        }
        return true;
    }

    pub fn fade_out(&mut self, fadeout_length: f64, mode: FadeMode) {
        let mut sample = *self.melody.last().unwrap();
        let time_length = sample.0;
        let mut i = self.melody.len() - 1;
        while (time_length - fadeout_length) < sample.0 {
            match mode {
                FadeMode::Linear => {
                    self.melody[i].1 = sample.1 * (time_length - sample.0) / fadeout_length;
                    i -= 1;
                    sample = self.melody[i];
                }
            }
        }
    }
    pub fn fade_in(&mut self, fadein_length: f64, mode: FadeMode) {
        let mut sample = self.melody[0];
        let start = sample.0;
        let mut i = 0;
        while (start + fadein_length) > sample.0 {
            match mode {
                FadeMode::Linear => {
                    self.melody[i].1 = sample.1 * (sample.0 - start) / fadein_length;
                    i += 1;
                    sample = self.melody[i];
                }
            }
        }
    }
    fn interfade(self, another: &mut Melody) {
        unimplemented!()
    }
}

// Note is a struct that contains data about >>>main<<< frequency of a sound,
// it's length and when it starts. Different instruments will produce different soundwaves.

pub struct Note {
    // Frequency can define note by itself. It's the thing that really matters. In Hz.
    freq: f64,
    // Measured in seconds, time from start of a note 'till it's quiet again. In seconds.
    leng: f64,
    // Time from the start of the melody, when this should be played, in seconds.
    time: f64,
}

impl Note {
    pub fn new(freq: f64, leng: f64, time: f64) -> Note {
        Note { freq, leng, time }
    }

    pub fn next(&self, freq: f64, leng: f64) -> Note {
        Note::new(freq, leng, self.time + self.leng)
    }
    pub fn default_melody() -> Vec<Note> {
        let first: Note = Note::new(440. * (-3.0_f64 / 4.0_f64).exp2(), 1., 0.5);
        let second = first.next(440. * (-1.0_f64 / 3.0_f64).exp2(), 1.);
        let third = second.next(440. * (-1.0_f64 / 6.0_f64).exp2(), 1.);
        let fourth = third.next(440. * (-1.0_f64 / 3.0_f64).exp2(), 1.);
        let fifth = fourth.next(440. * (-3.0_f64 / 4.0_f64).exp2(), 1.);
        let melody: Vec<Note> = vec![first, second, third, fourth, fifth];
        melody
    }
}

// Instruments are compilation of methods and coefficients that turn notes into soundwaves
// Simplest one is a sinewave.

pub trait Instrument {
    fn melody_from_notes(part: &Vec<Note>) -> Melody;
}

// Just a sinewave

pub struct SineWave {}

impl SineWave {
    pub fn gen_sine_wave_from_x0(
        freq: f64,
        t0: f64,
        t1: f64,
        x0: f64,
        deriv: f64,
    ) -> Vec<(f64, f64)> {
        assert!(t0 < t1, "t0 ({t0}) should be less then t1 ({t1})");
        let freq_r = freq * 2. * std::f64::consts::PI;
        let phase_sign = (x0 - deriv).signum();
        let phase_shift = if phase_sign > 0. {
            (x0).asin()
        } else {
            std::f64::consts::PI - (x0).asin()
        };
        let mut target_vector: Vec<(f64, f64)> = Vec::new();
        let times = math::linspace_from_n(t0, t1, (t1 - t0) as i64 * DESIRED_SAMPLE_RATE as i64);
        for i in times {
            target_vector.push((i, (phase_shift + freq_r * (i - t0)).sin()));
        }
        target_vector
    }
}

impl Instrument for SineWave {
    fn melody_from_notes(part: &Vec<Note>) -> Melody {
        let mut melody: Vec<(f64, f64)> = Vec::new();
        if !part.is_empty() {
            let mut x0 = 0.;
            let mut x_deriv = (0., 0.);
            for note in part {
                melody.append(&mut SineWave::gen_sine_wave_from_x0(
                    note.freq,
                    note.time,
                    note.leng + note.time,
                    x0,
                    x_deriv.1,
                ));
                x0 = melody.pop().unwrap_or((0., 0.)).1;
                x_deriv = *melody.last().unwrap_or(&(0., 0.));
            }
        }
        Melody { melody }
    }
}
