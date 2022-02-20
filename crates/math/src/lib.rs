#![feature(const_ptr_offset)]
#![feature(const_ptr_read)]

pub use matrix3x4::Matrix3x4;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4_aligned::Vec4Aligned;

mod matrix3x4;
mod vec2;
mod vec3;
mod vec4_aligned;
