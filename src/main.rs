use std::ffi::OsString;
use wav_gen::director;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    let path = std::env::args().nth(1).expect("no filename given");
    assert!(
        std::path::Path::new(&path).exists(),
        "given filename does not exist"
    );
    director(&OsString::from(path)).unwrap();
}
