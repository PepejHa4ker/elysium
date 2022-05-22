use core::mem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Instance {
    All = 0,
    Desktop = 1,
    Console = 2,
    Web = 4,
}

impl Instance {
    pub const fn from_i32(instance: i32) -> Option<Self> {
        Self::from_u64(instance as u64)
    }

    pub const fn from_u64(instance: u64) -> Option<Self> {
        match instance {
            0 => Some(Instance::All),
            1 => Some(Instance::Desktop),
            2 => Some(Instance::Console),
            3 => Some(Instance::Web),
            _ => None,
        }
    }

    pub const unsafe fn from_u64_unchecked(instance: u64) -> Self {
        mem::transmute(instance as i32)
    }

    pub const fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub const fn to_u64(&self) -> u64 {
        *self as u64
    }
}
