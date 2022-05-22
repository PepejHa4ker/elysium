use core::mem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Kind {
    Invalid = 0,
    Individual = 1,
    Multiseat = 2,
    GameServer = 3,
    AnonGameServer = 4,
    Pending = 5,
    ContentServer = 6,
    Clan = 7,
    Chat = 8,
    ConsoleUser = 9,
    AnonUser = 10,
}

impl Kind {
    pub const fn from_i32(kind: i32) -> Option<Self> {
        Self::from_u64(kind as u64)
    }

    pub const fn from_u64(kind: u64) -> Option<Self> {
        match kind {
            0 => Some(Kind::Invalid),
            1 => Some(Kind::Individual),
            2 => Some(Kind::Multiseat),
            3 => Some(Kind::GameServer),
            4 => Some(Kind::AnonGameServer),
            5 => Some(Kind::Pending),
            6 => Some(Kind::ContentServer),
            7 => Some(Kind::Clan),
            8 => Some(Kind::Chat),
            9 => Some(Kind::ConsoleUser),
            10 => Some(Kind::AnonUser),
            _ => None,
        }
    }

    pub const unsafe fn from_u64_unchecked(kind: u64) -> Self {
        mem::transmute(kind as i32)
    }

    pub const fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub const fn to_u64(&self) -> u64 {
        *self as u64
    }
}
