use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct FPSTimer {
    micros_per_frame: i32,
    last_time: Option<SystemTime>,
}

impl FPSTimer {
    pub fn new(max_fps: Option<f32>) -> Self {
        match max_fps {
            None => FPSTimer {
                micros_per_frame: 0,
                last_time: None,
            },
            Some(fps) => FPSTimer {
                micros_per_frame: (1_000_000.0 / fps) as i32,
                last_time: None,
            },
        }
    }

    pub fn frame(&mut self) -> u64 {
        let now = SystemTime::now();

        let mut dtime: u64 = 0;
        if self.last_time.is_some() {
            dtime = now.duration_since(self.last_time.unwrap()).unwrap().as_micros() as u64;
            let remaining_frame_time: i32 = self.micros_per_frame - dtime as i32;

            if remaining_frame_time > 0 {
                dtime += remaining_frame_time as u64;
                sleep(Duration::from_micros(remaining_frame_time as u64));
            }
        }

        self.last_time = Some(now);

        return dtime;
    }
}