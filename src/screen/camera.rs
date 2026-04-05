use macroquad::math::{Vec2, Vec3, vec3};

pub struct CameraPos {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
}

impl Default for CameraPos {
    fn default() -> Self {
        CameraPos {
            pos: vec3(0.0, 12.69, 17.57),
            target: vec3(0.0, 0.5, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 44.33, // 45.0,
        }
    }
}

impl CameraPos {
    pub fn set_zoom_rel(&mut self, coef: f32) {
        let target = self.target();
        let pos = self.pos();
        let new_pos = (pos - target) * coef + target;
        self.set_pos(new_pos);
    }
    pub fn get_zoom(&self) -> f32 {
        (self.pos - self.target).length()
    }
    pub fn pos(&self) -> Vec3 {
        self.pos
    }
    pub fn set_pos(&mut self, new_pos: Vec3) {
        self.pos = new_pos;
    }
    pub fn set_pos_rel(&mut self, delta: Vec2) {
        let zoom = self.get_zoom();
        set_rel(&mut self.pos, delta, zoom);
    }
    pub fn set_target_rel(&mut self, delta: Vec2) {
        let zoom = self.get_zoom();
        set_rel(&mut self.target, delta, zoom);
    }
    pub fn up(&self) -> Vec3 {
        self.up
    }
    pub fn target(&self) -> Vec3 {
        self.target
    }
    pub fn rotate(&mut self, delta: Vec2) {
        if delta.x.abs() > 0.01 || delta.y.abs() > 0.01 {
            let rotate_speed = 30.0;
            let front = self.target - self.pos;
            let zoom = front.length();
            let right = front.cross(self.up).normalize();

            let new_front =
                front - right * delta.x * rotate_speed + self.up * delta.y * rotate_speed;
            let new_front_same_zoom = new_front.normalize() * zoom;
            self.up = right.cross(new_front_same_zoom).normalize();
            self.pos = self.target - new_front_same_zoom;
        }
    }
}
pub fn set_rel(p: &mut Vec3, delta: Vec2, zoom: f32) {
    let move_speed = 0.32 * zoom;
    p.z += delta.y * move_speed;
    p.x += delta.x * move_speed;
}
