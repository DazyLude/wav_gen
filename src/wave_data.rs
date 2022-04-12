use crate::math::linerp_from_sample_rate;
use crate::track::DESIRED_SAMPLE_RATE;

pub trait WaveData {
    //getters
    fn get_size_in_bytes(&self) -> u32;
    fn get_bits_per_sample(&self) -> u16;
    fn get_encoding(&self) -> u16;
    fn len(&self) -> usize;
    fn to_byte_slice(&self) -> Vec<u8>;
    fn push_sample_data_from_f64(&mut self, data: f64);
    fn generate_from_wave(&mut self, wave: &Vec<f64>, sample_rate: u32) {
        let data_f64 = linerp_from_sample_rate(wave.to_vec(), DESIRED_SAMPLE_RATE, sample_rate);
        for f_val in data_f64 {
            assert!(
                f_val.abs() <= 1.,
                "wave amplitude is not within [-1, 1] range: {f_val}"
            );
            self.push_sample_data_from_f64(f_val);
        }
    }
    fn new() -> Self;
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
    fn get_encoding(&self) -> u16 {
        1
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn push_sample_data_from_f64(&mut self, data: f64) {
        self.push(((data + 1.) * 127.) as u8);
    }
    fn new() -> Self {
        let temp_vec: Vec<u8> = Vec::new();
        return temp_vec;
    }
}

impl WaveData for Vec<i16> {
    fn get_size_in_bytes(&self) -> u32 {
        self.len() as u32 * 2
    }
    fn get_bits_per_sample(&self) -> u16 {
        16
    }
    fn get_encoding(&self) -> u16 {
        1
    }
    fn len(&self) -> usize {
        self.len()
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
    fn push_sample_data_from_f64(&mut self, data: f64) {
        self.push((data * 32760.) as i16);
    }
    fn new() -> Self {
        let temp_vec: Vec<i16> = Vec::new();
        return temp_vec;
    }
}

impl WaveData for Vec<f32> {
    fn get_size_in_bytes(&self) -> u32 {
        self.len() as u32 * 4
    }
    fn get_bits_per_sample(&self) -> u16 {
        32
    }
    fn get_encoding(&self) -> u16 {
        3
    }
    fn len(&self) -> usize {
        self.len()
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
    fn push_sample_data_from_f64(&mut self, data: f64) {
        self.push(data as f32);
    }
    fn new() -> Self {
        let temp_vec: Vec<f32> = Vec::new();
        return temp_vec;
    }
}
