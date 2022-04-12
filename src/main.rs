use std::ffi::OsString;
use wav_gen::director;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    director(&OsString::from("examples/example_melody.wavg"), true).unwrap();
}
