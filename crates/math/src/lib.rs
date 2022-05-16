#![feature(const_float_classify)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

pub use matrix3x4::Matrix3x4;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4_aligned::Vec4Aligned;

mod matrix3x4;
mod vec2;
mod vec3;
mod vec4_aligned;
