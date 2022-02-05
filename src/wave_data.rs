use crate::math::linerp_vector_from_freq;

pub trait WaveData {
    //getters
    fn get_size_in_bytes(&self) -> u32;
    fn get_bits_per_sample(&self) -> u16;
    fn get_encoding(&self) -> u16;
    fn len(&self) -> usize;

    fn to_byte_slice(&self) -> Vec<u8>;
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32);
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
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        let data_f64 = linerp_vector_from_freq(wave, sample_rate as f64);
        for f_val in data_f64 {
            assert!(
                f_val.abs() <= 1.,
                "wave amplitude is not within [-1, 1] range"
            );
            self.push(((f_val + 1.) * 127.) as u8);
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
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        let data_f64 = linerp_vector_from_freq(wave, sample_rate as f64);
        for f_val in data_f64 {
            assert!(
                f_val.abs() <= 1.,
                "wave amplitude is not within [-1, 1] range"
            );
            self.push((f_val * 32760.) as i16);
        }
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
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        let data_f64 = linerp_vector_from_freq(wave, sample_rate as f64);
        for f_val in data_f64 {
            assert!(
                f_val.abs() <= 1.,
                "wave amplitude is not within [-1, 1] range"
            );
            self.push(f_val as f32);
        }
    }
}
