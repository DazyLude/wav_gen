use std::ffi::OsString;
use wav_gen::director;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    // std::env::args();
    director(&OsString::from("examples/example_melody.wavg"), false).unwrap();
}
// у меня мейн аккуратненький, инвалид, ничего не делает, все по канонам
