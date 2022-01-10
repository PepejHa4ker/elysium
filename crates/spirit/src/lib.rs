#![allow(incomplete_features)]
#![feature(const_fn_trait_bound)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(crate_visibility_modifier)]
#![feature(generic_const_exprs)]
#![feature(extern_types)]
#![feature(ptr_metadata)]
#![no_std]

mod slice;
mod str;

crate mod opaque;
crate mod util;

pub use crate::slice::Slice;
pub use crate::str::Str;
