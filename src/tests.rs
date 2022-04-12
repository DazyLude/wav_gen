use crate::*;
use std::str::FromStr;
#[test]
fn note_parsing() {
    // "A4 0 1/4"
    let a4 = harmonics::MelodicNote {
        octave: 4,
        tone: harmonics::ToneName::A,
        variant: harmonics::ToneVariant::None,
        delta: (0, 1),
        length: (1, 4),
    };
    // "C3 Flat 1/4 1/4"
    let c3 = harmonics::MelodicNote {
        octave: 3,
        tone: harmonics::ToneName::C,
        variant: harmonics::ToneVariant::Flat,
        delta: (1, 4),
        length: (1, 4),
    };
    // "d5 3/2 1/8"
    let d5 = harmonics::MelodicNote {
        octave: 5,
        tone: harmonics::ToneName::D,
        variant: harmonics::ToneVariant::Sharp,
        delta: (3, 2),
        length: (1, 8),
    };
    assert_eq!(a4, harmonics::MelodicNote::from_str("A4 0 1/4").unwrap());
    assert_eq!(
        c3,
        harmonics::MelodicNote::from_str("C3 Flat 1/4 1/4").unwrap()
    );
    assert_eq!(
        d5,
        harmonics::MelodicNote::from_str("d5 sharp 3/2 1/8").unwrap()
    );
}
#[test]
fn note_parsing_gibberish() {
    let a4 = harmonics::MelodicNote {
        octave: 4,
        tone: harmonics::ToneName::A,
        variant: harmonics::ToneVariant::None,
        delta: (0, 1),
        length: (1, 4),
    };
    assert_eq!(
        a4,
        harmonics::MelodicNote::from_str("A 440 hz please").unwrap()
    );
}
