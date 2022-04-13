// Harmonics turn notes that a person can easily percieve,
// such as C-sharp (kappa) or D-flats into a collection of frequencies and other data
// used by intruments in intruments.rs

// string -----------------------| make_note() |---------------------------> note
// under the hood (even if note_type is an empty struct):
// string --|from_str(&str)|-> note_type --|to_note(note_type, vec<pars>)|-> note
// note type is defined by the instrument
// e.g sine uses melodic note, and clicks use hit note

pub trait MakeNote {
    fn make_note(s: &str, pars: Vec<f64>) -> crate::instruments::Note;
}

pub enum NoteType {
    MelodicNote,
}

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

fn par_check(length: usize, expected: usize) {
    if length != expected {
        panic!("incorrect parsed parameters, expected {expected}, got {length}");
    }
}

#[derive(PartialEq, Debug)]
pub struct MelodicNote {}

fn parse_unwrap(s: &str) -> i64 {
    match s.parse::<i64>() {
        Ok(val) => val,
        Err(e) => panic!("Failed parsing note: {e}"),
    }
}

impl MakeNote for MelodicNote {
    fn make_note(s: &str, pars: Vec<f64>) -> crate::instruments::Note {
        // parameter 1: BPM in 1/4th per minute
        // parameter 2: 1/4th since the beginning of the melody until this bar
        par_check(pars.len(), 2);

        let mut split: Vec<&str> = s.split(' ').collect();

        let mut pop_to_fraq = || -> (i64, i64) {
            let s = split.pop().unwrap();
            if s.contains('/') {
                let temp_s: Vec<&str> = s.split('/').collect();
                (parse_unwrap(temp_s[0]), parse_unwrap(temp_s[1]))
            } else {
                (parse_unwrap(s), 1)
            }
        };

        // Length of the note
        let length = pop_to_fraq();
        // Time after the beginning of the bar
        let delta = pop_to_fraq();
        // Standard names for the tones, but can also be an integer (in that case C = 0, D = 1, etc up to B = 6)
        let tone: ToneName = match &split[0][0..1] {
            "b" => ToneName::B,
            "c" => ToneName::C,
            "d" => ToneName::D,
            "e" => ToneName::E,
            "f" => ToneName::F,
            "g" => ToneName::G,
            "a" => ToneName::A,
            _ => panic!("Failed while parsing note: unknown tone tame"),
        };
        // octave number. A440 is in the 4th one. Can be negative.
        let octave: i64 = parse_unwrap(split[0].get(1..).unwrap());
        // flats and sharps. Can be an integrer, negatives - flats, positives - sharps
        let mut variant = ToneVariant::None;
        // if split length is not 2, then variant is implied
        if split.len() == 2 {
            variant = match split.pop().unwrap() {
                "flat" => ToneVariant::Flat,
                "sharp" => ToneVariant::Sharp,
                val => {
                    if val.parse::<f64>().is_ok() {
                        ToneVariant::Numeric(parse_unwrap(val))
                    } else {
                        ToneVariant::None
                    }
                }
            };
        }

        // semitones is a distance, in semitones, from A4
        let mut semitones: i64;
        semitones = (octave - 4) * 12;
        semitones += match tone {
            ToneName::C => -9,
            ToneName::D => -7,
            ToneName::E => -5,
            ToneName::F => -4,
            ToneName::G => -2,
            ToneName::A => 0,
            ToneName::B => 2,
        };
        semitones += match variant {
            ToneVariant::Flat => -1,
            ToneVariant::None => 0,
            ToneVariant::Sharp => 1,
            ToneVariant::Numeric(val) => val,
        };
        crate::instruments::Note::new(
            440. * (semitones as f64 / 12.).exp2(),             // freq
            length.0 as f64 / length.1 as f64 * 240. / pars[0], // leng
            (pars[1] + 4. * delta.0 as f64 / delta.1 as f64) * 60. / pars[0], // time
        )
    }
}
