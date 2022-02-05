use wav_gen::gen_sine_wave;
use wav_gen::gen_wav_file;
use wav_gen::wave_data::WaveData;
use wav_gen::WavConfig;

fn main() {
    // Generating a sound to turn into a *.wav file, funniest shit I've ever seen
    let wave = gen_sine_wave(440., 10.);

    // Creating a wav file from the generated sound
    let mut data: Vec<f32> = Vec::new();
    data.generate_from_wave(wave, 88000);
    let cfg: WavConfig<Vec<f32>> = WavConfig::new("test.wav".to_string(), 1, 88000, data);
    gen_wav_file(cfg);
}
