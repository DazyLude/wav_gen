use wav_gen::gen_wav_file;
use wav_gen::instruments::Instrument;
use wav_gen::instruments::Note;
use wav_gen::instruments::SineWave;
use wav_gen::track::FadeMode;
use wav_gen::track::Track;
use wav_gen::wave_data::WaveData;

use wav_gen::WavConfig;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    let mut sine: Track = SineWave::track_from_notes(&Note::default_track());
    sine.fade_in_mask(0.5, FadeMode::Linear);
    sine.fade_out_mask(0.5, FadeMode::Linear);
    sine.normalize();
    sine.apply_loudness();

    // Creating a wav file from the generated sound
    let mut data: Vec<i16> = Vec::new();
    data.generate_from_wave(&sine.track, 44100);
    let cfg: WavConfig<Vec<i16>> = WavConfig::new("test.wav".to_string(), 1, 44100, data);
    gen_wav_file(cfg);
}
