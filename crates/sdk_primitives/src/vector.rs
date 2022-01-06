use crate::{Angle, Matrix3x4};
use core::ops::{Add, Div, Mul, Sub};
use vek::vec::repr_simd::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// create a new vector
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// zero... lol
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// calculate the magnitude of this vector
    pub fn magnitude(self) -> f32 {
        Vec3::new(self.y, self.x, self.y).magnitude()
    }

    /// calculate the magnitude without squaring
    pub fn magnitude_squared(self) -> f32 {
        Vec3::new(self.y, self.x, self.y).magnitude_squared()
    }

    /// calculate the magnitude of y and x of this vector
    pub fn magnitude2d(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude()
    }

    /// calculate the 2d magnitude without squaring
    pub fn magnitude2d_squared(self) -> f32 {
        Vec2::new(self.y, self.x).magnitude_squared()
    }

    pub fn to_angle(self) -> Angle {
        let mut yaw = 0.0;
        let mut pitch = 0.0;

        if self.x == 0.0 && self.y == 0.0 {
            if self.z > 0.0 {
                pitch = 270.0;
            } else {
                pitch = 90.0;
            }
        } else {
            yaw = self.y.atan2(self.x).to_degrees();

            if yaw < 0.0 {
                yaw += 360.0;
            }

            let x = self.x.hypot(self.y);

            pitch = (-self.z).atan2(x).to_degrees();

            if pitch < 0.0 {
                pitch += 360.0;
            }
        }

        Angle::new(pitch, yaw)
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.to_vec3().dot(rhs.to_vec3())
    }

    pub fn rotate_xy(self, origin: Self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        let mut this = self;

        this.x -= origin.x;
        this.y -= origin.y;

        let new_x = this.x * cos - this.y * sin;
        let new_y = this.x * sin + this.y * cos;

        this.x = new_x + origin.x;
        this.y = new_x + origin.y;
        this
    }

    pub fn angle(self, up: Self) -> Vector {
        let forward = self;
        let left = up * forward;
        let forward_distance = forward.magnitude();
        let mut angle = Self::zero();

        if forward_distance > 0.001 {
            let up_z = (left.y * forward.x) - (left.x * forward.y);

            angle.x = (-forward.z).atan2(forward_distance).to_degrees();
            angle.y = forward.z.atan2(forward.x).to_degrees();
            angle.z = left.z.atan2(up_z).to_degrees();
        } else {
            angle.x = (-forward.z).atan2(forward_distance).to_degrees();
            angle.y = (-left.x).atan2(left.y).to_degrees();
            angle.z = 0.0;
        }

        angle
    }

    pub fn rotate(self, rhs: Self) -> Self {
        let angled_matrix = Matrix3x4::from_angle(Angle {
            pitch: rhs.x,
            yaw: rhs.y,
            roll: rhs.z,
        });

        let x = rhs.dot(angled_matrix.x());
        let y = rhs.dot(angled_matrix.y());
        let z = rhs.dot(angled_matrix.z());

        Vector::new(x, y, z)
    }

    fn to_vec3(self) -> Vec3<f32> {
        Vec3::new(self.x, self.y, self.z)
    }

    fn from_vec3(Vec3 { x, y, z }: Vec3<f32>) -> Self {
        Self::new(x, y, z)
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_vec3(self.to_vec3() + rhs.to_vec3())
    }
}

impl Add<f32> for Vector {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self::from_vec3(self.to_vec3() + rhs)
    }
}

impl Div<Self> for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::from_vec3(self.to_vec3() / rhs.to_vec3())
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::from_vec3(self.to_vec3() / rhs)
    }
}

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_vec3(self.to_vec3() * rhs.to_vec3())
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::from_vec3(self.to_vec3() * rhs)
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_vec3(self.to_vec3() - rhs.to_vec3())
    }
}

impl Sub<f32> for Vector {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self::from_vec3(self.to_vec3() - rhs)
    }
}
