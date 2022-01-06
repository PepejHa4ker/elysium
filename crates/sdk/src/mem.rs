use core::mem;
use core::mem::MaybeUninit;

/// Reinterprets the bits of a value of one type as another type.
///
/// Unlike `core::mem::transmute`, this doesn't care about sizes.
///
/// # Safety
///
/// There is no safety.
pub const unsafe fn transmute<T, U>(value: T) -> U {
    let transmuted = mem::transmute_copy(&value);

    mem::forget(value);

    transmuted
}

/// Create a new array of `T` items, in an uninitialized state.
pub const fn uninit_array<T, const LEN: usize>() -> [T; LEN] {
    unsafe { self::transmute(MaybeUninit::<T>::uninit_array::<LEN>()) }
}

/// Create a new uninit `T`.
pub const fn uninit<T>() -> T
where
    [(); mem::size_of::<T>()]:,
{
    unsafe { self::transmute(self::uninit_array::<u8, { mem::size_of::<T>() }>()) }
}

/// Create a new zeroed `T`.
pub const fn zeroed<T>() -> T
where
    [(); mem::size_of::<T>()]:,
{
    unsafe { self::transmute([0u8; mem::size_of::<T>()]) }
}
