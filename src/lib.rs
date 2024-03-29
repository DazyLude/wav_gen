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


use crate::harmonics::MakeNote;
use crate::track::Track;
use crate::wave_data::WaveData;
use std::ffi::OsString;
use std::fs;
use std::io::Write;
use instruments::InstrumentList;
use instruments::Instrument;

// black magic that makes BufRead work
use std::io::prelude::*;
use std::io::BufReader;



// .wavg file interpretator and main routine

pub fn director(wavg_filename: &OsString) -> std::io::Result<()> {
    let file = fs::File::open(wavg_filename)?;
    let file = BufReader::new(file);
    let mut counter: i64 = 0;
    let mut player: InstrumentList = InstrumentList::None;
    let mut player_pars: Vec<(String, String)> = Vec::new(); 
    let mut notes: Vec<instruments::Note> = Vec::new();
    let mut global_pars: GlobalParameters = GlobalParameters::new_default();
    let mut track: Track = track::Track::new();

    fn cut_with_colon(split: &str, counter: i64) -> (String, String) {
        match split.find(':') {
            None => panic!("wavg synthax error: no colon in parameter definition at line {}", counter),
            Some(colon) => (split.get(0..colon).unwrap().trim().to_string(),split.get(colon+1..).unwrap().trim().to_string()),
        }
    }

    fn unwrap_update (res: Result<(), &str>, counter: i64) {
        match res {
            Err(e) => panic!("you passed a wrong parameter, buddy: {e}, at line: {counter}"),
            Ok(()) => {}
        }
    }

    fn get_note_type (instrument: &InstrumentList, counter: i64) -> harmonics::NoteType {
        match instrument {
            InstrumentList::None => panic!(
                "wavg synthax error: parsing notes before defining an instrument at line {counter}"),
            InstrumentList::SineWave => harmonics::NoteType::MelodicNote,
            InstrumentList::Xylophone => harmonics::NoteType::MelodicNote,
        }
    }

    let frac = |x: (i64, i64)| -> f64 {x.0 as f64 / x.1 as f64};

    fn record<T: Instrument>(pars: &Vec<(String, String)>, notes: &Vec<instruments::Note>, counter: i64) -> track::Track {
        let mut player: T = T::new();
        for par in pars.clone() { 
            unwrap_update(player.update(&par), counter);
        }
        player.track_from_notes(&notes)
    }

    // Parser "the cursed" edition
    for wrapped in file.lines() {
        counter += 1;
        let line = wrapped.unwrap().trim().to_ascii_lowercase().to_string();
        // Commentaries and empty lines are being ignored
        if line.get(0..1) == Some("#") || line == "" {
            continue;
        }
        match (line.find(':'), line.find(',')) {
            //keyword lines have neither colons nor commas
            (None, None) => { 
                match line.as_str() {
                    // "record" flushes notes into instrument
                    "record" => {
                        match player {
                            InstrumentList::None => 
                                panic!("wavg synthax error: parsing notes before defining an instrument at line {counter}"),
                            InstrumentList::SineWave => 
                                track = track.mix(&mut record::<instruments::SineWave>(&player_pars, &notes, counter)),
                            InstrumentList::Xylophone => 
                                track = track.mix(&mut record::<instruments::Xylophone>(&player_pars, &notes, counter)),    
                        }
                        notes = Vec::new();
                    }
                    // "end" finishes this comedy
                    "end" => break,
                    _ => panic!("wavg synthax error: not a keyword at line {counter}"),
                }
            }
            //notebar lines don't have colons
            (None, Some(first_comma_pos)) => {
                let bar_index = match line.get(..first_comma_pos).unwrap().trim().parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => panic!("wavg synthax error: incorrect bar number at line {counter}"),
                };
                let bar_timing = (bar_index - 1) as f64 * 4. * frac(global_pars.time_signature);

                match get_note_type(&player, counter) {
                    harmonics::NoteType::MelodicNote => {
                        for element in line.get(first_comma_pos + 1..).unwrap().split(',') {
                            notes.push(
                                harmonics::MelodicNote::make_note(
                                    element.trim(),
                                    Vec::from([global_pars.beats_per_minute, bar_timing])
                                )
                            );
                        }
                    }
                }
            }
            // config lines have colons, commas are optional
            (Some(first_colon), first_comma_option) => {
                match line.get(0..first_colon).unwrap().trim() 
                {
                    // "notesheet" keyword defines the instrument used for the following bars
                    "notesheet" => {
                        let first_comma: usize = match first_comma_option{
                            None => line.len(),
                            Some(val) => val,
                        };

                        match line.get(first_colon + 1..first_comma).unwrap().trim() 
                        {
                            "sinewave" => player = InstrumentList::SineWave,
                            "simpledrum" => player = InstrumentList::Xylophone,
                            _ => panic!("wavg synthax error: instrument not found; line {counter}"),
                        }

                        // else, there should be instrument config
                        for entry in line.get(first_comma..).unwrap().split(',') {
                            // if this was just an instrument declaration, we can go on
                            if entry == "" {continue;}
                            player_pars.push(cut_with_colon(entry, counter));
                        }
                    }
                    // if not a notesheet, then a global config line
                    _ => {
                        for entry in line.split(',') {
                            unwrap_update(global_pars.update(&cut_with_colon(entry, counter)), counter);
                        };
                    }
                }
            }
        }
    }
    track.start_with_silence();
    track.normalize();
    track.apply_loudness();

    let mut this_file_name = format!("{}.wav", global_pars.name);
    let mut this_file_index = 0; 
    while std::path::Path::new(&this_file_name).exists() {
        this_file_name = format!("{}({}).wav", global_pars.name, this_file_index); 
        this_file_index += 1;
    }
    global_pars.name = this_file_name;

    match global_pars.bits_per_sample {
        8 => {
            global_pars.generate_wav_file::<Vec<u8>>(&track);
        },
        16 => {
            global_pars.generate_wav_file::<Vec<i16>>(&track);
        },
        32 => {
            global_pars.generate_wav_file::<Vec<f32>>(&track);
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
}

impl GlobalParameters {
    fn new_default() -> GlobalParameters {
        GlobalParameters {
            name: "wave_generator_generated.wav".to_string(),
            sample_rate: 44100,
            bits_per_sample: 16,
            beats_per_minute: 120.,
            time_signature: (4, 4),
        }
    }

    fn generate_wav_file<T: WaveData> (&self, datatrack: &track::Track) {
        let mut data = T::new();
        data.generate_from_wave(&datatrack.track, self.sample_rate);
        let cfg: WavConfig<T> = WavConfig::new(self.name.clone(), 1, self.sample_rate, data);
        gen_wav_file(cfg);
    }

    fn update<'a>(&mut self, param: &'a (String, String)) -> Result<(), &'a str> {
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
            _ => {
                return Err(param.0.as_str());
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
