use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32, // радианы, 0 = смотрим вдоль -Z
    pub pitch: f32,
    pub speed: f32, // единиц в секунду
}

#[derive(Default)]
pub struct Input {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

const SENSITIVITY: f32 = 0.0025;

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            yaw: 0.0,
            pitch: 0.0,
            speed: 4.0,
        }
    }

    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            -self.yaw.cos() * self.pitch.cos(),
        )
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        // движение по горизонтали, без учёта наклона вверх/вниз
        let flat_forward = Vec3::new(self.yaw.sin(), 0.0, -self.yaw.cos());
        let right = flat_forward.cross(Vec3::Y);

        let mut dir = Vec3::ZERO;
        if input.forward {
            dir += flat_forward;
        }
        if input.back {
            dir -= flat_forward;
        }
        if input.right {
            dir += right;
        }
        if input.left {
            dir -= right;
        }
        if input.up {
            dir += Vec3::Y;
        }
        if input.down {
            dir -= Vec3::Y;
        }

        if dir != Vec3::ZERO {
            self.position += dir.normalize() * self.speed * dt;
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * SENSITIVITY;
        self.pitch -= dy * SENSITIVITY;
        let limit = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn view(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position, self.position + self.forward(), Vec3::Y)
    }
}
