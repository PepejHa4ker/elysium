#![allow(incomplete_features)]
#![feature(const_transmute_copy)]
#![feature(const_refs_to_cell)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]

pub use self::animation_layer::AnimationLayer;
pub use self::animation_state::AnimationState;
pub use self::pad::Pad;
pub use sdk_primitives::*;

mod animation_layer;
mod animation_state;
mod mem;
mod pad;
