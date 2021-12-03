#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]

pub use self::angle::Angle;
pub use self::f32ext::F32Ext;
pub use self::matrix3x4::Matrix3x4;
pub use self::vector::Vector;
pub use self::vector2d::Vector2D;

mod angle;
mod f32ext;
mod matrix3x4;
mod vector;
mod vector2d;
