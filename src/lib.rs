mod math;
pub mod wave_data;

use crate::wave_data::WaveData;
use std::ffi::OsString;
use std::fs;
use std::io::Seek;
use std::io::Write;

pub struct WavConfig<T: WaveData> {
    file_name: String,
    chunk_size: u32,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk_2_size: u32,
    sound: T,
}

impl<T: WaveData> WavConfig<T> {
    pub fn new(file_name: String, num_channels: u16, sample_rate: u32, sound: T) -> WavConfig<T> {
        let subchunk_2_size: u32 =
            (sound.len() as u32 - 1) * sound.get_bits_per_sample() as u32 / 8;
        let bits_per_sample: u16 = sound.get_bits_per_sample();

        let chunk_size: u32 = subchunk_2_size + 36;
        let byte_rate: u32 = sample_rate * bits_per_sample as u32 * num_channels as u32 / 8;
        let block_align: u16 = bits_per_sample / 8 * num_channels;

        WavConfig {
            file_name,
            chunk_size,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            subchunk_2_size,
            sound,
        }
    }
}

pub fn gen_wav_file<T: WaveData>(cfg: WavConfig<T>) {
    let mut file = match fs::OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(OsString::from(&cfg.file_name))
    {
        Ok(result) => result,
        Err(e) => panic!("{}", e),
    };

    file.write(&0x52_49_46_46_i32.to_be_bytes()).unwrap_or(0); // "RIFF"
    file.write(&cfg.chunk_size.to_le_bytes()).unwrap_or(0);
    file.write(&0x57_41_56_45_i32.to_be_bytes()).unwrap_or(0); // "WAVE"
    file.write(&0x66_6d_74_20_i32.to_be_bytes()).unwrap_or(0); // "fmt "
    file.write(&0x00_00_00_10_i32.to_le_bytes()).unwrap_or(0); // 16
    file.write(&0x00_01_i16.to_le_bytes()).unwrap_or(0); // PCM
    file.write(&cfg.num_channels.to_le_bytes()).unwrap_or(0);
    file.write(&cfg.sample_rate.to_le_bytes()).unwrap_or(0);
    file.write(&cfg.byte_rate.to_le_bytes()).unwrap_or(0);
    file.write(&cfg.block_align.to_le_bytes()).unwrap_or(0);
    file.write(&cfg.bits_per_sample.to_le_bytes()).unwrap_or(0);
    file.write(&0x64_61_74_61_i32.to_be_bytes()).unwrap_or(0); // "0 Data"
    file.write(&cfg.subchunk_2_size.to_le_bytes()).unwrap_or(0);
    file.write(&cfg.sound.to_byte_slice()).unwrap_or(0);

    let size = file.metadata().unwrap().len();
    file.seek(std::io::SeekFrom::Start(4)).unwrap();
    file.write(&(size as u32 - 8).to_le_bytes()).unwrap_or(0);
    file.seek(std::io::SeekFrom::Start(40)).unwrap();
    file.write(&(size as u32 - 44).to_le_bytes()).unwrap_or(0);
}

pub fn gen_sine_wave(freq: f64, length: f64) -> Vec<(f64, f64)> {
    let mut target_vector: Vec<(f64, f64)> = Vec::new();
    let times = math::linspace2(0.0, length, 10000);
    for i in times {
        target_vector.push((i, (freq * i * 2. * std::f64::consts::PI).sin()));
    }
    println!("Generated sine wave");
    target_vector
}
