pub const A440: f64 = 440.;

// Harmonics turn notes that a person can easily percieve,
// such as C-sharp (kappa) or D-flats into a collection of frequencies and other data

pub trait Harmonics {
    fn get_note(&self, note: NoteNote) -> Note;
    // fn get_note(&self, freq: f64) -> NoteNote;
}

pub enum ToneName {
    c,
    d,
    e,
    f,
    g,
    h,
    b,
    Numeric(i32),
}

pub enum ToneVariant {
    bb,
    b,
    None,
    d,
    dd,
    Numeric(i32),
}

pub struct NoteNote {
    // thats a thing to implement
    // you can use standard note nomenclature. ways to interpret it can vary, but I used the simplest one.

    // octave number. A440 is in the 4th one. Can be negative.
    // Beware that most people won't be able to hear almost anything from -1st and less and 10th and higher
    octave_n: i32,
    // Standard names for the tones, but can also be an integer (in that case C = 0, D = 1, etc up to B = 6)
    tone: ToneName,
    // Standart flats and sharps, up to 2. Can be an integrer, negatives - flats, positives - sharps
    variants: ToneVariant,
    // Raw modificators for frequency
    raw_add: f64,
    raw_mul: f64,
}
