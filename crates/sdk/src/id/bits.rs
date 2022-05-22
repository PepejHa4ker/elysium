const fn mask(bits: u8) -> u64 {
    (1 << bits) - 1
}

#[cfg(target_endian = "big")]
pub const ID_SHIFT: u32 = 32;
#[cfg(target_endian = "big")]
pub const INSTANCE_SHIFT: u32 = 12;
#[cfg(target_endian = "big")]
pub const KIND_SHIFT: u32 = 8;
#[cfg(target_endian = "big")]
pub const UNIVERSE_SHIFT: u32 = 0;

#[cfg(target_endian = "little")]
pub const ID_SHIFT: u32 = 0;
#[cfg(target_endian = "little")]
pub const INSTANCE_SHIFT: u32 = 32;
#[cfg(target_endian = "little")]
pub const KIND_SHIFT: u32 = 52;
#[cfg(target_endian = "little")]
pub const UNIVERSE_SHIFT: u32 = 56;

pub const ID_MASK: u64 = mask(32);
pub const INSTANCE_MASK: u64 = mask(20);
pub const KIND_MASK: u64 = mask(4);
pub const UNIVERSE_MASK: u64 = mask(8);
