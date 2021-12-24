#![feature(const_fn_trait_bound)]
#![feature(const_ptr_offset)]
#![feature(const_refs_to_cell)]
#![feature(const_transmute_copy)]

use core::mem;

pub use self::signature::Signature;

mod signature;

pub const unsafe fn transmute<T, U>(value: T) -> U {
    let result = mem::transmute_copy(&value);

    mem::forget(value);

    result
}

pub const unsafe fn transmute_fn<F>(address: *const ()) -> F
where
    F: Signature,
{
    transmute(address)
}

pub const unsafe fn get<F>(address: *const (), offset: usize) -> F
where
    F: Signature,
{
    transmute_fn(*((address as *const u8).add(offset) as *const *const ()))
}
