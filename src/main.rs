use wav_gen::gen_sine_wave;
use wav_gen::gen_wav_file;
use wav_gen::wave_data::WaveData;
use wav_gen::WavConfig;

fn main() {
    // Creating a sound turn into a *.wav file, funniest shit I've ever seen
    let wave = gen_sine_wave(440., 2.);

    // Creating a wav file from the sound
    let mut data: Vec<i16> = Vec::new();
    data.generate_from_wave(wave, 44100);
    let cfg: WavConfig<Vec<i16>> = WavConfig::new("test.wav".to_string(), 1, 44100, data);
    gen_wav_file(cfg);
}
