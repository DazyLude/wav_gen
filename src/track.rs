pub const DESIRED_SAMPLE_RATE: u32 = 44100;

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

impl Track {
    pub fn new() -> Track {
        let track: Vec<f64> = Vec::new();
        Track {
            track,
            starting_sample_index: 0,
            loudness: 1.,
        }
    }
    pub fn length(&self) -> f64 {
        self.track.len() as f64 / DESIRED_SAMPLE_RATE as f64
    }
    pub fn ending_sample_index(&self) -> usize {
        self.track.len() + self.starting_sample_index
    }
    /// returns an absolute sample index of a given abolute time, or an amount of samples in a given timeframe
    pub fn time_to_sample_index(t0: f64) -> usize {
        (t0 * DESIRED_SAMPLE_RATE as f64).trunc() as usize
    }
    pub fn sample_index_to_time(i: usize) -> f64 {
        i as f64 / DESIRED_SAMPLE_RATE as f64
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
}

pub trait Mask {
    fn apply(self, track: &mut Track);
}

pub struct LinearFadeInOut {
    length: f64,
    is_out: bool,
}

impl LinearFadeInOut {
    pub fn r#in() -> Self {
        LinearFadeInOut {
            length: 0.2,
            is_out: false,
        }
    }
    pub fn in_l(length: f64) -> Self {
        LinearFadeInOut {
            length: length,
            is_out: false,
        }
    }
    pub fn out() -> Self {
        LinearFadeInOut {
            length: 0.2,
            is_out: true,
        }
    }
    pub fn out_l(length: f64) -> Self {
        LinearFadeInOut {
            length: length,
            is_out: true,
        }
    }
}

impl Mask for LinearFadeInOut {
    fn apply(self, track: &mut Track) {
        let (x0, x1): (f64, f64) = if self.is_out {
            (track.length(), track.length() - self.length)
        } else {
            (0.0, self.length)
        };
        let (quiet_sample, loud_sample): (usize, usize) = (
            Track::time_to_sample_index(x0),
            Track::time_to_sample_index(x1),
        );
        let linear = |x_i| -> f64 {
            (x_i as f64 - quiet_sample as f64) / (loud_sample as f64 - quiet_sample as f64)
        };
        let range = if (quiet_sample..loud_sample).is_empty() {
            loud_sample..quiet_sample
        } else {
            quiet_sample..loud_sample
        };
        for i in range {
            track.track[i] *= linear(i);
        }
    }
}
