use crate::math::lininter;
use crate::math::linspace;

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
        // unzips wave from a vector of coordinates to two vectors for each axis
        let (time, amplitude): (Vec<f64>, Vec<f64>) = wave.into_iter().unzip();
        // creting a vector for the desired sample times
        let mut desired_times: Vec<f64> = Vec::new();
        linspace(
            &mut desired_times,
            time[0],
            time[time.len()],
            1. / sample_rate as f64,
        );

        let mut i = 0;
        let mut k = 0;
        let mut f_val: f64;

        while i < desired_times.len() {
            if time[k] < desired_times[i] {
                k = k + 1;
            }
            f_val = lininter(
                (time[k], amplitude[k]),
                (time[k + 1], amplitude[k + 1]),
                desired_times[i],
            ) * 256.;
            //pushing a linearly interpolated value corresponding to a desired sample time
            self.push(f_val.trunc() as u8);
            i = i + 1;
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
        // unzips wave from a vector of coordinates to two vectors for each axis
        let (time, amplitude): (Vec<f64>, Vec<f64>) = wave.into_iter().unzip();
        // creting a vector for the desired sample times
        let mut desired_times: Vec<f64> = Vec::new();
        linspace(
            &mut desired_times,
            time[0],
            time[time.len() - 1],
            sample_rate as f64,
        );

        let mut i = 0;
        let mut k = 0;
        let mut f_val: f64;

        while (i < desired_times.len() - 1) && (k < time.len() - 1) {
            if time[k + 1] < desired_times[i] {
                k = k + 1;
            }
            f_val = lininter(
                (time[k], amplitude[k]),
                (time[k + 1], amplitude[k + 1]),
                desired_times[i],
            );
            if f_val.abs() >= 1. {
                f_val = f_val.trunc();
            }
            f_val *= 32760.;
            println!("{}", f_val as i16);
            //pushing a linearly interpolated value corresponding to a desired sample time
            self.push(f_val.trunc() as i16);
            i = i + 1;
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
    fn to_byte_slice(&self) -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        for sample in self {
            for byte in sample.to_be_bytes() {
                vector.push(byte);
            }
        }
        vector
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn generate_from_wave(&mut self, wave: Vec<(f64, f64)>, sample_rate: u32) {
        // unzips wave from a vector of coordinates to two vectors for each axis
        let (time, amplitude): (Vec<f64>, Vec<f64>) = wave.into_iter().unzip();
        // creting a vector for the desired sample times
        let mut desired_times: Vec<f64> = Vec::new();
        linspace(
            &mut desired_times,
            time[0],
            time[time.len()],
            1. / sample_rate as f64,
        );

        let mut i = 0;
        let mut k = 0;

        while i < desired_times.len() {
            if time[k] < desired_times[i] {
                k = k + 1;
            }
            //pushing a linearly interpolated value corresponding to a desired sample time
            self.push(lininter(
                (time[k], amplitude[k]),
                (time[k + 1], amplitude[k + 1]),
                desired_times[i],
            ) as f32);
            i = i + 1;
        }
    }
}
