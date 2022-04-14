pub const DESIRED_SAMPLE_RATE: u32 = 16000;

pub struct Track {
    pub track: Vec<f64>,
    pub starting_sample_index: usize,
    pub loudness: f64,
}

impl From<Vec<f64>> for Track {
    fn from(thing: Vec<f64>) -> Track {
        Track {
            track: thing,
            starting_sample_index: 0,
            loudness: 1.,
        }
    }
}

impl From<Track> for Vec<f64> {
    fn from(thing: Track) -> Vec<f64> {
        thing.track
    }
}

impl From<Vec<Track>> for Track {
    fn from(mut thing: Vec<Track>) -> Track {
        let mut temp: Track = Track::new();
        for i in 0..thing.len() {
            temp = temp.mix(&mut thing[i]);
        }
        temp
    }
}

impl From<Track> for Vec<(f64, f64)> {
    fn from(thing: Track) -> Vec<(f64, f64)> {
        println!(
            "This function (from(Track) -> Vec<(f64, f64)>) was called somewhere. Consider reworking it to work with Vec<f64> instead"
        );
        let mut wave: Vec<(f64, f64)> = Vec::new();
        let mut i: usize = 0;
        for sample in thing.track {
            wave.push((
                sample,
                (thing.starting_sample_index + i) as f64 / DESIRED_SAMPLE_RATE as f64,
            ));
            i += 1;
        }
        wave
    }
}

pub enum FadeMode {
    Linear,
    Exponential,
}

impl Track {
    pub fn new() -> Track {
        let track: Vec<f64> = Vec::new();
        Track {
            track,
            starting_sample_index: 0,
            loudness: 1.,
        }
    }
    pub fn t_end(&self) -> f64 {
        (self.track.len() + self.starting_sample_index) as f64 / DESIRED_SAMPLE_RATE as f64
    }
    pub fn length(&self) -> f64 {
        self.track.len() as f64 / DESIRED_SAMPLE_RATE as f64
    }
    pub fn ending_sample_index(&self) -> usize {
        self.track.len() + self.starting_sample_index
    }
    pub fn start_with_silence(&mut self) {
        let beginning = vec![0.; self.starting_sample_index];
        self.track = [beginning, self.track.clone()].concat();
    }
    pub fn sample_in_global(&self, i: usize) -> f64 {
        if i > self.starting_sample_index && i - self.starting_sample_index < self.track.len() {
            self.track[i - self.starting_sample_index]
        } else {
            0.
        }
    }
    pub fn get_value_at_t(&self, sample_time: f64) -> f64 {
        if self.track.len() == 0 {
            return 0.;
        }
        let mut sampling_sample = (sample_time * DESIRED_SAMPLE_RATE as f64).floor() as usize;
        if sampling_sample < self.starting_sample_index {
            return 0.;
        }
        sampling_sample -= self.starting_sample_index;
        if sampling_sample > self.track.len() {
            return 0.;
        }
        self.track[sampling_sample - 1]
    }
    pub fn get_deriv_at_t(&self, sample_time: f64) -> f64 {
        if self.track.len() == 0 {
            return 0.;
        }
        let mut sampling_sample = (sample_time * DESIRED_SAMPLE_RATE as f64).floor() as usize - 1;
        if sampling_sample < self.starting_sample_index {
            return 0.;
        }
        sampling_sample -= self.starting_sample_index;
        if sampling_sample > self.track.len() {
            return 0.;
        }
        self.track[sampling_sample - 1]
    }

    pub fn mix(&mut self, another: &mut Track) -> Track {
        //true values represent self partially covering another and self starting earlier
        let mix_starting_sample_index = self
            .starting_sample_index
            .min(another.starting_sample_index);
        let mix_ending_sample_index = self
            .ending_sample_index()
            .max(another.ending_sample_index());

        let mut mix: Vec<f64> =
            Vec::with_capacity(mix_ending_sample_index - mix_starting_sample_index);
        for i in mix_starting_sample_index..mix_ending_sample_index {
            mix.push(
                self.sample_in_global(i) * self.loudness
                    + another.sample_in_global(i) * another.loudness,
            );
        }
        Track {
            track: mix,
            starting_sample_index: mix_starting_sample_index,
            loudness: 1.,
        }
    }

    pub fn cut(&mut self, t0: f64, t1: f64) {
        let t0_sample = (DESIRED_SAMPLE_RATE as f64 * t0).trunc() as usize;
        let t1_sample = (DESIRED_SAMPLE_RATE as f64 * t1).trunc() as usize;
        let mut temp_track: Vec<f64> = Vec::new();
        for i in t0_sample..t1_sample {
            temp_track.push(self.sample_in_global(i));
        }
        self.track = temp_track;
        self.starting_sample_index = t0_sample;
    }

    // makes everything to be within [-1; 1] range
    pub fn normalize(&mut self) {
        let mut max_amp: f64 = 0.;
        for sample in &self.track {
            if max_amp < sample.abs() {
                max_amp = sample.abs();
            }
        }
        self.loudness = 1. / max_amp;
    }

    pub fn apply_loudness(&mut self) {
        for sample in &mut self.track {
            *sample *= self.loudness;
        }
    }

    pub fn bell_mask(&mut self, middle: f64, sigma: f64) {
        if middle <= 0.0 && sigma <= 0.0 {
            panic!("trying to apply bell mask with sigma: {sigma} and middle: {middle}");
        }
        let middle_sample = DESIRED_SAMPLE_RATE as f64 * middle;
        let width_in_samples = DESIRED_SAMPLE_RATE as f64 * sigma;
        for i in 0..self.track.len() {
            self.track[i] *= (-0.5 * ((i as f64 - middle_sample) / width_in_samples).powi(2)).exp()
        }
        self.cut(middle - sigma, middle + sigma);
        self.fade_in_mask(sigma / 2., FadeMode::Linear);
        self.fade_out_mask(sigma / 2., FadeMode::Linear);
    }

    pub fn fade_out_mask(&mut self, fadeout_length: f64, mode: FadeMode) {
        let len = self.track.len();
        let mut i = len - 1;
        let fadeout_samples = (fadeout_length * DESIRED_SAMPLE_RATE as f64).floor() as usize;
        while i > len - fadeout_samples {
            match mode {
                FadeMode::Linear => {
                    self.track[i] *= (len - i) as f64 / fadeout_samples as f64;
                }
                FadeMode::Exponential => {
                    self.track[i] *= 2.
                        - (2.0_f64.ln() * (i + fadeout_samples - len) as f64
                            / fadeout_samples as f64)
                            .exp()
                }
            }
            i -= 1;
        }
    }
    pub fn fade_in_mask(&mut self, fadein_length: f64, mode: FadeMode) {
        let mut i = 0;
        let fadein_samples = (fadein_length * DESIRED_SAMPLE_RATE as f64).floor() as usize;
        while fadein_samples > i {
            match mode {
                FadeMode::Linear => {
                    self.track[i] *= i as f64 / fadein_samples as f64;
                }
                FadeMode::Exponential => {
                    self.track[i] *= 2.
                        - (2.0_f64.ln() * (fadein_samples - i) as f64 / fadein_samples as f64).exp()
                }
            }
            i += 1;
        }
    }
}
