use macroquad::math::{Vec3, vec3};

pub struct CameraPos {
    pub y: f32,
    pub z: f32,
    pub fovy: f32,
    pub target_y: f32,
}

impl Default for CameraPos {
    fn default() -> Self {
        CameraPos {
            y: 12.69,      // 6.0,
            z: 17.57,      // 8.0,
            fovy: 44.33,   // 45.0,
            target_y: 0.5, // 0.0,
        }
    }
}

impl CameraPos {
    pub fn set_zoom(&mut self) {}
    pub fn get_zoom(&self) {}
    pub fn pos(&self) -> Vec3 {
        vec3(0.0, self.y, self.z)
    }
    pub fn up(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }
    pub fn target(&self) -> Vec3 {
        vec3(0.0, self.target_y, 0.0)
    }
}
