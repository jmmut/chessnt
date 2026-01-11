use crate::FPS_AVERAGE_FRAMES;
use macroquad::miniquad::date::now;

pub struct Time {
    pub current_s: f64,
    pub last_s: f64,
    pub frame_count: i32,
    pub rolling_frame_time: f64,
    pub cached_fps: f64,
}

impl Time {
    pub fn new() -> Self {
        let now = now();
        Self {
            current_s: now - 1.0 / 60.0,
            last_s: now - 1.0 / 60.0,
            frame_count: 0,
            rolling_frame_time: 0.0,
            cached_fps: 0.0,
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
    pub fn fps(&self) -> f64 {
        self.cached_fps
    }
}
