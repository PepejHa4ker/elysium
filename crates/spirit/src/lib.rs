#![allow(incomplete_features)]
#![feature(const_precise_live_drops)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(extern_types)]
#![feature(ptr_metadata)]
#![no_std]

mod slice;
mod str;

pub(crate) mod opaque;
pub(crate) mod util;

pub use crate::slice::Slice;
pub use crate::str::Str;
