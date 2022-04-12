use std::num::ParseIntError;
use std::str::FromStr;

// Harmonics turn notes that a person can easily percieve,
// such as C-sharp (kappa) or D-flats into a collection of frequencies and other data
// used by intruments in intruments.rs

#[derive(PartialEq, Debug)]
pub enum ToneName {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

#[derive(PartialEq, Debug)]
pub enum ToneVariant {
    Flat,
    None,
    Sharp,
    Numeric(i64),
}

pub trait ToNote {
    fn to_note(&self, pars: Vec<f64>) -> crate::instruments::Note;
}

fn par_check(length: usize, expected: usize) {
    if length != expected {
        panic!("incorrect parsed parameters, expected {expected}, got {length}");
    }
}

#[derive(PartialEq, Debug)]
pub struct MelodicNote {
    // thats a thing to implement
    // you can use standard note nomenclature. ways to interpret it can vary, but I used the simplest one.

    // octave number. A440 is in the 4th one. Can be negative.
    // Beware that most people won't be able to hear almost anything from -1st and less and 10th and higher
    pub octave: i64,
    // Standard names for the tones, but can also be an integer (in that case C = 0, D = 1, etc up to B = 6)
    pub tone: ToneName,
    // Standart flats and sharps, up to 2. Can be an integrer, negatives - flats, positives - sharps
    pub variant: ToneVariant,
    // Timings, when to play this note and how long it should be
    // Time, in fraction of a beat, after the beginning of the bar
    pub delta: (i64, i64),
    // Length, in fraction of a beat, of the note
    pub length: (i64, i64),
}

impl MelodicNote {
    pub fn make_note(s: &str, pars: Vec<f64>) -> crate::instruments::Note {
        MelodicNote::from_str(s).unwrap().to_note(pars)
    }
}

impl ToNote for MelodicNote {
    fn to_note(&self, pars: Vec<f64>) -> crate::instruments::Note {
        // parameter 1: BPM in 1/4th per minute
        // parameter 2: 1/4th since the beginning of the melody until this bar
        par_check(pars.len(), 2);
        // semitones is a distance, in semitones, from A4
        let mut semitones: i64;
        semitones = (self.octave - 4) * 12;
        semitones += match self.tone {
            ToneName::C => -9,
            ToneName::D => -7,
            ToneName::E => -5,
            ToneName::F => -4,
            ToneName::G => -2,
            ToneName::A => 0,
            ToneName::B => 2,
        };
        semitones += match self.variant {
            ToneVariant::Flat => -1,
            ToneVariant::None => 0,
            ToneVariant::Sharp => 1,
            ToneVariant::Numeric(val) => val,
        };
        crate::instruments::Note::new(
            440. * (semitones as f64 / 12.).exp2(),                // freq
            pars[0] * self.length.0 as f64 / self.length.1 as f64, // leng
            pars[0] * (pars[1] + 4. * self.delta.0 as f64 / self.delta.1 as f64), // time
        )
    }
}

impl FromStr for MelodicNote {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let tone: ToneName = match &split[0][0..1] {
            "b" | "B" => ToneName::B,
            "c" | "C" => ToneName::C,
            "d" | "D" => ToneName::D,
            "e" | "E" => ToneName::E,
            "f" | "F" => ToneName::F,
            "g" | "G" => ToneName::G,
            "a" | "A" | _ => ToneName::A,
        };
        let octave: i64 = split[0].get(1..).unwrap().parse::<i64>()?;

        let (variant, variant_implied): (ToneVariant, usize) = if split.len() == 4 {
            (
                match split[1] {
                    "flat" | "Flat" => ToneVariant::Flat,
                    "sharp" | "Sharp" => ToneVariant::Sharp,
                    _ => ToneVariant::None,
                },
                1,
            )
        } else {
            (ToneVariant::None, 0)
        };
        let delta: (i64, i64);
        if split[1 + variant_implied].contains('/') {
            let delta_intern: Vec<&str> = split[1 + variant_implied].split('/').collect();
            delta = (
                delta_intern[0].parse::<i64>()?,
                delta_intern[1].parse::<i64>()?,
            );
        } else {
            delta = (split[1 + variant_implied].parse::<i64>()?, 1);
        };
        let length: (i64, i64);
        if split[2 + variant_implied].contains('/') {
            let length_intern: Vec<&str> = split[2 + variant_implied].split('/').collect();
            length = (
                length_intern[0].parse::<i64>()?,
                length_intern[1].parse::<i64>()?,
            );
        } else {
            length = (split[2 + variant_implied].parse::<i64>()?, 1);
        };

        return Ok(MelodicNote {
            octave,
            tone,
            variant,
            delta,
            length,
        });
    }
}
