use crate::math::linerp_vector_from_freq;

pub trait WaveData {
    fn get_size_in_bytes(&self) -> u32;
    fn get_bits_per_sample(&self) -> u16;
    fn to_byte_slice(&self) -> Vec<u8>;
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32);
    fn len(&self) -> usize;
}

impl WaveData for Vec<u8> {
    fn get_size_in_bytes(&self) -> u32 {
        self.len() as u32
    }
    fn get_bits_per_sample(&self) -> u16 {
        8
    }
    fn to_byte_slice(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        for sample in self {
            vector.push(sample.to_be_bytes()[0])
        }
        vector
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        if self.len() != 0 {
            println!("generating into a non-empty array");
        }
        let data_f64 = linerp_vector_from_freq(wave, sample_rate as f64);
        //pushing a linearly interpolated value corresponding to a desired sample time
        for f_val in data_f64 {
            self.push(((f_val + 1.) * 128.).trunc() as u8);
        }
    }
}

impl WaveData for Vec<i16> {
    fn get_size_in_bytes(&self) -> u32 {
        self.len() as u32 * 2
    }
    fn get_bits_per_sample(&self) -> u16 {
        16
    }
    fn to_byte_slice(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        for sample in self {
            for byte in sample.to_le_bytes() {
                vector.push(byte);
            }
        }
        vector
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        if self.len() != 0 {
            println!("generating into a non-empty array");
        }
        // unzips wave from a vector of coordinates to two vectors for each axis
        let data_f64 = linerp_vector_from_freq(wave, sample_rate as f64);
        for f_val in data_f64 {
            self.push((f_val * 32760.).trunc() as i16);
        }
    }
}
