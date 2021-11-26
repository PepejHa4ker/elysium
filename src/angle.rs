use vek::vec::repr_simd::Vec2;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Angle {
    pub pitch: f32,
    pub yaw: f32,
    _roll: f32,
}

impl Angle {
    pub const fn new(pitch: f32, yaw: f32) -> Self {
        Self {
            pitch,
            yaw,
            _roll: 0.0,
        }
    }

    pub fn magnitude(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude()
    }

    pub fn magnitude_squared(self) -> f32 {
        Vec2::new(self.pitch, self.yaw).magnitude_squared()
    }

    pub fn normalize_pitch(self) -> Self {
        Self::new(self.pitch % 89.0, self.yaw)
    }

    pub fn normalize_yaw(self) -> Self {
        Self::new(self.pitch, self.yaw % 180.0)
    }

    pub fn normalize(self) -> Self {
        Self::new(self.pitch % 89.0, self.yaw % 180.0)
    }
}
