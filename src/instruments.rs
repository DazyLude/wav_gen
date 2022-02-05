use crate::math;

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
}

pub trait Instrument {
    fn generate_melody(&self, party: &Vec<Note>) -> Vec<(f64, f64)>;
}

pub struct SineWave {}

impl SineWave {
    pub fn gen_sine_wave(freq: f64, length: f64) -> Vec<(f64, f64)> {
        let mut target_vector: Vec<(f64, f64)> = Vec::new();
        let times = math::linspace_from_n(0.0, length, length as i64 * 10000);
        for i in times {
            target_vector.push((i, (freq * i * 2. * std::f64::consts::PI).sin()));
        }
        println!("Generated sine wave");
        target_vector
    }
    fn add(sine1: &mut Vec<(f64, f64)>, sine2: &mut Vec<(f64, f64)>) {
        let time_add = sine1.last().unwrap_or(&(0., 0.)).0;
        sine1.pop();
        for sample in sine2 {
            sample.0 += time_add;
            sine1.push(*sample);
        }
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

impl Instrument for SineWave {
    fn generate_melody(&self, party: &Vec<Note>) -> Vec<(f64, f64)> {
        let mut melody: Vec<(f64, f64)> = Vec::new();
        let mut generated: Vec<(f64, f64)>;
        for note in party {
            generated = SineWave::gen_sine_wave(note.freq, note.leng);
            SineWave::add(&mut melody, &mut generated);
        }
        melody
    }
}
