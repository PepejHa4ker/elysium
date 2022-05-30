use core::mem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Universe {
    Invalid = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4,
}

impl Universe {
    #[inline]
    pub const fn from_i32(universe: i32) -> Option<Self> {
        Self::from_u64(universe as u64)
    }

    #[inline]
    pub const fn from_u64(universe: u64) -> Option<Self> {
        match universe {
            0 => Some(Universe::Invalid),
            1 => Some(Universe::Public),
            2 => Some(Universe::Beta),
            3 => Some(Universe::Internal),
            4 => Some(Universe::Dev),
            _ => None,
        }
    }

    #[inline]
    pub const unsafe fn from_u64_unchecked(universe: u64) -> Self {
        mem::transmute(universe as i32)
    }

    #[inline]
    pub const fn to_i32(&self) -> i32 {
        *self as i32
    }

    #[inline]
    pub const fn to_u64(&self) -> u64 {
        *self as u64
    }
}
