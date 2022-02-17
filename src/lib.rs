mod math;
pub mod track;
pub mod wave_data;

// notesheet should be passed to harmonics, then they give the note structs
// then the orchestrator dispenses note-vectors to the instruments
// and then they return sound data, which is passed to WaveData, which generates .wav file
pub mod instruments;

use crate::wave_data::WaveData;
use std::ffi::OsString;
use std::fs;
use std::io::Write;

pub struct WavConfig<T: WaveData> {
    file_name: String,
    chunk_size: u32,
    encoding: u16,
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
        let subchunk_2_size: u32 = (sound.len() as u32) * sound.get_bits_per_sample() as u32 / 8;
        let bits_per_sample: u16 = sound.get_bits_per_sample();
        let encoding: u16 = sound.get_encoding();

        let chunk_size: u32 = subchunk_2_size + 36;
        let byte_rate: u32 = sample_rate * bits_per_sample as u32 * num_channels as u32 / 8;
        let block_align: u16 = bits_per_sample / 8 * num_channels;

        WavConfig {
            file_name,
            chunk_size,
            encoding,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            subchunk_2_size,
            sound,
        }
    }

    pub fn assemble_header(&self) -> [u8; 44] {
        let mut header: [u8; 44] = [0; 44];
        let mut i = 0;

        // "RIFF"
        for byte in 0x52_49_46_46_i32.to_be_bytes() {
            header[i] = byte;
            i += 1;
        }
        // size of the file - 8
        for byte in self.chunk_size.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // "WAVE" (be) + "fmt " (be) + size of format subchunk (le)

        for byte in [
            0x57, 0x41, 0x56, 0x45, 0x66, 0x6d, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00,
        ] {
            header[i] = byte;
            i += 1;
        }
        // encoding: PCM (1) or smth else. Data can be stored with floats and then it's not PCM
        for byte in self.encoding.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // number of channels, mono or stereo or smth else
        for byte in self.num_channels.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // sample rate - samples per second
        for byte in self.sample_rate.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // byte rate - average bytes per second
        for byte in self.byte_rate.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // block align - how many bytes per sample for all channels
        for byte in self.block_align.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // block align - how many bits per sample for one channel
        for byte in self.bits_per_sample.to_le_bytes() {
            header[i] = byte;
            i += 1;
            // "DATA"
        }
        for byte in 0x64_61_74_61_i32.to_be_bytes() {
            header[i] = byte;
            i += 1;
        }
        // size of the data
        for byte in self.subchunk_2_size.to_le_bytes() {
            header[i] = byte;
            i += 1;
        }
        // End of header

        header
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
        Err(e) => panic!("Could not open the file: {}", e),
    };

    match file.write(&cfg.assemble_header()) {
        Ok(result) => {
            if result != 44 {
                panic!(
                    "Was expecting to write 44 bytes to the file, wrote {}",
                    result
                );
            }
        }
        Err(e) => panic!("Could not write header to the file: {}", e),
    }

    match file.write(&cfg.sound.to_byte_slice()) {
        Ok(result) => {
            if result != cfg.subchunk_2_size as usize {
                panic!(
                    "Was epecting to write {} bytes to the file, wrote {}",
                    cfg.subchunk_2_size, result
                );
            }
        }
        Err(e) => panic!("Could not write data to the file: {}", e),
    }
}
