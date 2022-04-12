pub mod harmonics;
mod math;
#[cfg(test)]
mod tests;
pub mod track;
pub mod wave_data;

// notesheet should be passed to harmonics, then they give the note structs
// then the orchestrator dispenses note-vectors to the instruments
// and then they return sound data, which is passed to WaveData, which generates .wav file
pub mod instruments;

use crate::instruments::Instrument;
use crate::track::Track;
use crate::wave_data::WaveData;
use std::ffi::OsString;
use std::fs;
use std::io::Write;


// black magic that makes BufRead work
use std::io::prelude::*;
use std::io::BufReader;

// .wavg file interpretator and main routine

pub fn director(wavg_filename: &OsString, debug: bool) -> std::io::Result<()> {
    let file = fs::File::open(wavg_filename)?;
    let file = BufReader::new(file);
    let mut counter: i64 = 0;
    let mut player: instruments::InstrumentList = instruments::InstrumentList::None;
    let mut player_pars: Vec<(String, String)> = Vec::new(); 
    let mut notes: Vec<instruments::Note> = Vec::new();
    let mut global_pars: GlobalParameters = GlobalParameters::new_default();
    let mut track: Track = track::Track::new();
  
    // Parser "the cursed" edition
    for wrapped in file.lines() {
        counter += 1;
        let line = wrapped.unwrap().trim().to_string();
        // Commentaries are being ignored
        if line.get(0..1) == Some("#") || line == "" {
            if debug {println!("skipped line {counter}");}
            continue;
        }
        match line.find(':') {
            // The notebar and special keyword lines don't have colons
            None => {
                // keywords:
                // "record" flushes notes into instrument
                if line == "record" {
                    if debug {println!("recording at line {counter}");}
                    match player {
                        instruments::InstrumentList::None => 
                            panic!("wavg synthax error: parsing notes before defining an instrument at line {counter}"),
                        instruments::InstrumentList::SineWave => {
                            let mut sine = instruments::SineWave::new();
                            for par in &player_pars { 
                                match sine.set_parameter(&par.0.as_str(), &par.1.as_str()) {
                                    Err(s) => panic!("Failed setting a parameter {s} for SineWave: parameter does not exist."),
                                    Ok(_) => {}
                                }
                            }
                            track = track.mix(&mut sine.track_from_notes(&notes));
                            notes = Vec::new();
                        }
                    }
                    continue;
                }
                // "end" finishes this comedy
                if line == "end" {
                    if debug {println!("end at line {counter}");}
                    break;
                }
                // if this is not a keyword, then it is a notebar
                if line.find(',') == None {
                    panic!("wavg synthax error: no commas in bar definition at line {counter}");
                }
                let first_comma_position = line.find(',').unwrap();

                let bar_index = line
                    .get(..first_comma_position)
                    .unwrap()
                    .trim()
                    .parse::<i64>()
                    .unwrap();
                if debug {println!("parsing bar {bar_index}:");}
                match player {
                    instruments::InstrumentList::None => {
                        panic!("wavg synthax error: parsing notes before defining an instrument at line {counter}");
                    }
                    instruments::InstrumentList::SineWave => {
                        for element in line.get(first_comma_position + 1..).unwrap().split(',') {
                            notes.push(
                                // this gets a note string, converts it to melodic note, and then converts melodic note to note
                                harmonics::MelodicNote::make_note(element.trim(),Vec::from(
                                            [global_pars.beats_per_minute,
                                            (bar_index - 1) as f64 * 4. * global_pars.time_signature.0 as f64 / global_pars.time_signature.1 as f64]
                                    )
                                )
                            );
                            println!("parsed note: {}, behold my might!", notes.last().unwrap());
                        }
                    }
                }
            }
            // if colon was found, then it's a configuration line
            Some(first_colon_position) => { 
                // checking if the first word is a keyword
                match line
                    .get(0..first_colon_position)
                    .unwrap()
                    .trim()
                    .to_ascii_lowercase()
                    .as_str()
                {
                    // Notesheet: instrument, param1: value1, param2: value2, param3: value3
                    // "notesheet" keyword defines the instrument used for the following bars
                    "notesheet" => {
                        let first_comma_position = line.trim().find(',').unwrap_or_else(||(line.trim().len() - 1) as usize);
                        match line
                            .get(first_colon_position + 1..first_comma_position)
                            .unwrap()
                            .trim()
                            .to_ascii_lowercase()
                            .as_str()
                        {
                            "sinewave" => player = instruments::InstrumentList::SineWave,
                            _ => panic!("wavg synthax error: instrument not found; line {counter}"),
                        }
                        // else, there should be instrument config
                        for entry in line.get(first_comma_position..).unwrap().split(',') {
                            // if this was just an instrument declaration, we can go on
                            if entry == "" {continue;}
                            match entry.find(':') {
                                None => panic!("wavg synthax error: no colon in global config definition at line {counter}"),
                                Some(colon) => {
                                    let name = entry.get(0..colon).unwrap().trim().to_string();
                                    let val = entry.get(colon+1..).unwrap().trim().to_string();
                                    if debug {println!("passing {name} with a value: {val} to sinewave");}
                                    player_pars.push((name,val));
                                }
                            }
                        }
                    }
                    // Name: Example, BPM: 60, Time_Signature: 4/4
                    _ => {
                        for entry in line.split(',') {
                            match entry.find(':') {
                                None => panic!("wavg synthax error: no colon in global config definition at line {counter}"),
                                Some(colon) => {
                                    let name = entry.get(0..colon).unwrap().trim();
                                    let val = entry.get(colon+1..).unwrap().trim();
                                    if debug {println!("passing {name} with a value: {val} to global_parameters");}
                                    global_pars.update((name, val)).unwrap();
                                }
                            };
                        }
                    }
                }
            }
        }
    }
    track.normalize();
    track.apply_loudness();

    match global_pars.bits_per_sample {
        8 => {
            let mut data: Vec<u8> = Vec::new();
            data.generate_from_wave(&track.track, global_pars.sample_rate);
            let cfg: WavConfig<Vec<u8>> = WavConfig::new("test.wav".to_string(), 1, global_pars.sample_rate, data);
            gen_wav_file(cfg);
        },
        16 => {
            let mut data: Vec<i16> = Vec::new();
            data.generate_from_wave(&track.track, global_pars.sample_rate);
            let cfg: WavConfig<Vec<i16>> = WavConfig::new("test.wav".to_string(), 1, global_pars.sample_rate, data);
            gen_wav_file(cfg);
        },
        32 => {
            let mut data: Vec<f32> = Vec::new();
            data.generate_from_wave(&track.track, global_pars.sample_rate);
            let cfg: WavConfig<Vec<f32>> = WavConfig::new("test.wav".to_string(), 1, global_pars.sample_rate, data);
            gen_wav_file(cfg);
        },
        _ => panic!("unknown bits per sample setting: {}. Try 8, 16 or 32", global_pars.bits_per_sample)
    } 
    Ok(())
}

struct GlobalParameters {
    name: String,
    sample_rate: u32,
    bits_per_sample: u16,
    beats_per_minute: f64,
    time_signature: (i64, i64),
    fade_mode: track::FadeMode,
    fade_time: (i64, i64),
}

impl GlobalParameters {
    fn new_default() -> GlobalParameters {
        GlobalParameters {
            name: "wave_generator_generated.wav".to_string(),
            sample_rate: 44100,
            bits_per_sample: 16,
            beats_per_minute: 120.,
            time_signature: (4, 4),
            fade_mode: track::FadeMode::Linear,
            fade_time: (1, 32),
        }
    }
    fn update<'a>(&mut self, param: (&'a str, &'a str)) -> Result<(), &'a str> {
        match param.0.to_ascii_lowercase().as_str() {
            "name" => self.name = param.1.trim().to_string(),
            "samplerate" => self.sample_rate = param.1.trim().parse::<u32>().unwrap(),
            "bitspersample" => self.bits_per_sample = param.1.trim().parse::<u16>().unwrap(),
            "beatsperminute" | "bpm" => {
                self.beats_per_minute = param.1.trim().parse::<f64>().unwrap()
            }
            "time_signature" => {
                let fraction: Vec<i64> = param.1.trim().split('/').map(|s| s.parse::<i64>().unwrap()).collect();
                self.time_signature = (fraction[0], fraction[1]);
            }
            "fademode" => {
                self.fade_mode = match param.1.trim().to_ascii_lowercase().as_str() {
                    "linear" => track::FadeMode::Linear,
                    _ => return Err(param.1),
                }
            }
            "fadetime" => {
                let slash_position = param.1.find('/').unwrap();
                self.fade_time = (
                    param
                        .1
                        .get(..slash_position)
                        .unwrap()
                        .trim()
                        .parse::<i64>()
                        .unwrap(),
                    param
                        .1
                        .get(slash_position + 1..)
                        .unwrap()
                        .trim()
                        .parse::<i64>()
                        .unwrap(),
                );
            }
            _ => {
                return Err(param.0);
            }
        }
        
        return Ok(());
    }
}

// .wav file generation routines

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
