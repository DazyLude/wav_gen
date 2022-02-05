pub const A440: f64 = 440.;

pub trait Harmonics {
    fn get_frequency(&self, note: NoteNote) -> f64;
    // fn get_note(&self, freq: f64) -> NoteNote;
}

pub trait Instrument {
    fn generate_melody(&self, party: &Vec<Note>) -> Vec<f64>;
    fn update_melody(&self, party: &Vec<Note>);
}

pub struct Note {
    // Frequency can define note by itself, . It's the thing that really matters
    frequency: f64,
    // Length of the note is also a factor. Measured in seconds
    length: f64,
    // Time from the start of the melody, when this should be played, in seconds
    timing: f64,
}

pub impl Note {
    pub fn new(freq: f64, leng: f64, time: f64) -> Note {
        Note { freq, leng, time }
    }
}

pub enum ToneName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
    Numeric(i32),
}

pub enum ToneVariant {
    BB,
    B,
    None,
    D,
    DD,
    Numeric(i32),
}

pub struct NoteNote {
    // thats a thing to implement
    // you can also use standard note nomenclature. ways to interpret it can vary, but I used the simplest one.
    // octave number. A440 is in the 4th one. Can be negative.
    // Beware that most people won't be able to hear almost anything from -1st and less and 10th and higher
    octave_n: i32,
    // Standard names for the tones, but can also be an integer (in that case C = 0, D = 1, etc up to B = 6)
    // Can be negative
    tone: ToneName,
    // Standart bemoles and
    variants: ToneVariant,
    // Raw modificator for frequency
    raw_mod: f64,
}
