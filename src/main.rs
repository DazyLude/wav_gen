use wav_gen::gen_wav_file;
use wav_gen::instruments::Instrument;
use wav_gen::instruments::SineWave;
use wav_gen::wave_data::WaveData;

use wav_gen::WavConfig;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    let sine: SineWave = SineWave {};
    let wave = sine.generate_melody(&SineWave::default_melody());

    // Creating a wav file from the generated sound
    let mut data: Vec<i16> = Vec::new();
    data.generate_from_wave(wave, 44100);
    let cfg: WavConfig<Vec<i16>> = WavConfig::new("test.wav".to_string(), 1, 44100, data);
    gen_wav_file(cfg);
}
