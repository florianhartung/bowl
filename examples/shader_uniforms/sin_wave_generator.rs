pub struct SinWaveGenerator {
    total_advanced_time: u64,
}

impl SinWaveGenerator {
    pub fn new() -> Self {
        Self { total_advanced_time: 0 }
    }

    pub fn pass_time(&mut self, passed_time: u64) {
        self.total_advanced_time += passed_time;
    }

    pub fn calc_current(&self) -> f32 {
        (self.total_advanced_time as f32 / 1_000_000_f32).sin().powf(2.0)
    }
}
