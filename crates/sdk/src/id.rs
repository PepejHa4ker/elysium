pub use instance::Instance;
pub use kind::Kind;
pub use universe::Universe;

mod instance;
mod kind;
mod universe;

pub mod bits;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct SteamId(pub u64);

impl SteamId {
    pub const fn new(id: u64) -> Option<Self> {
        if Instance::from_u64(instance(id)).is_none() {
            return None;
        }

        if Kind::from_u64(kind(id)).is_none() {
            return None;
        }

        if Universe::from_u64(universe(id)).is_none() {
            return None;
        }

        Some(Self(id))
    }

    pub const unsafe fn new_unchecked(id: u64) -> Self {
        Self(id)
    }

    pub const fn id(&self) -> u32 {
        id(self.0) as u32
    }

    pub const fn instance(&self) -> Instance {
        unsafe { Instance::from_u64_unchecked(instance(self.0)) }
    }

    pub const fn kind(&self) -> Kind {
        unsafe { Kind::from_u64_unchecked(kind(self.0)) }
    }

    pub const fn universe(&self) -> Universe {
        unsafe { Universe::from_u64_unchecked(universe(self.0)) }
    }
}

pub const fn id(id: u64) -> u64 {
    (id >> bits::ID_SHIFT) & bits::ID_MASK
}

pub const fn instance(id: u64) -> u64 {
    (id >> bits::INSTANCE_SHIFT) & bits::INSTANCE_MASK
}

pub const fn kind(id: u64) -> u64 {
    (id >> bits::KIND_SHIFT) & bits::KIND_MASK
}

pub const fn universe(id: u64) -> u64 {
    (id >> bits::UNIVERSE_SHIFT) & bits::UNIVERSE_MASK
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEAMID: u64 = 76_561_199_254_102_667;
    const STEAMID3: u32 = 1_293_836_939;

    #[test]
    fn id() {
        let id = SteamId::new(STEAMID).unwrap();

        assert_eq!(id.id(), STEAMID3);
    }

    #[test]
    fn instance() {
        let id = SteamId::new(STEAMID).unwrap();

        assert_eq!(id.instance(), Instance::Desktop);
    }

    #[test]
    fn kind() {
        let id = SteamId::new(STEAMID).unwrap();

        assert_eq!(id.kind(), Kind::Individual);
    }

    #[test]
    fn universe() {
        let id = SteamId::new(STEAMID).unwrap();

        assert_eq!(id.universe(), Universe::Public);
    }
}
