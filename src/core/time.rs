use crate::FPS_AVERAGE_FRAMES;
use macroquad::miniquad::date::now;
use std::time::Duration;

pub const DEFAULT_FPS: f64 = 60.0;

pub struct Time {
    pub current_s: f64,
    pub last_s: f64,
    pub last_end_s: f64,
    pub frame_count: i32,
    pub rolling_frame_time: f64,
    pub cached_fps: f64,
    pub target_fps: Option<f64>,
}

impl Time {
    pub fn new() -> Self {
        Self::new_fps(None)
    }

    fn new_fps(fps: Option<f64>) -> Time {
        let now = now();
        Self {
            current_s: now - 1.0 / 60.0,
            last_s: now - 1.0 / 60.0,
            last_end_s: now - 1.0 / 60.0,
            frame_count: 0,
            rolling_frame_time: 0.0,
            cached_fps: 0.0,
            target_fps: fps,
        }
    }

    pub fn tick(&mut self) {
        self.frame_count = (self.frame_count + 1) % (1000 * FPS_AVERAGE_FRAMES);
        self.last_s = self.current_s;
        self.current_s = now();
        self.rolling_frame_time += self.current_s - self.last_s;
        if self.frame_count % FPS_AVERAGE_FRAMES == 0 {
            self.cached_fps = 1.0 / (self.rolling_frame_time / FPS_AVERAGE_FRAMES as f64);
            self.rolling_frame_time = 0.0;
        }
    }
    pub fn tick_end(&mut self) {
        #[cfg(not(target_family = "wasm"))]
        {
            if let Some(fps) = self.target_fps {
                let last = self.last_end_s;
                let now = now();
                self.last_end_s = now;
                let target_frame_time = 1.0 / fps;
                let frame_time = now - last;
                if frame_time < 0.9 * target_frame_time {
                    let to_sleep = ((0.99 * target_frame_time - frame_time) * 1000.0) as u64;
                    std::thread::sleep(Duration::from_millis(to_sleep));
                    self.last_end_s = macroquad::miniquad::date::now();
                }
            }
        }
    }
    pub fn delta(&self) -> f64 {
        self.current_s - self.last_s
    }
    pub fn fps(&self) -> f64 {
        self.cached_fps
    }
    pub fn set_target_fps(&mut self, fps: Option<f64>) {
        self.target_fps = fps;
    }
    pub fn get_target_fps(&self) -> Option<f64> {
        self.target_fps
    }
}
