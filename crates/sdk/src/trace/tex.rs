/// Texture identifiers?
#[repr(C)]
pub struct Tex(u8);

impl Tex {
    pub const ANTLION: Self = Self(b'A');
    pub const BLOODYFLESH: Self = Self(b'B');
    pub const CONCRETE: Self = Self(b'C');
    pub const DIRT: Self = Self(b'D');
    /// the egg sacs in the tunnels in ep2
    pub const EGGSHELL: Self = Self(b'E');
    pub const FLESH: Self = Self(b'F');
    pub const GRATE: Self = Self(b'G');
    pub const ALIENFLESH: Self = Self(b'H');
    pub const CLIP: Self = Self(b'I');
    // pub const UNUSED: Self = Self(b'J');
    pub const SNOW: Self = Self(b'K');
    pub const PLASTIC: Self = Self(b'L');
    pub const METAL: Self = Self(b'M');
    pub const SAND: Self = Self(b'N');
    pub const FOLIAGE: Self = Self(b'O');
    pub const COMPUTER: Self = Self(b'P');
    // pub const UNUSED: Self = Self(b'Q');
    pub const REFLECTIVE: Self = Self(b'R');
    pub const SLOSH: Self = Self(b'S');
    pub const TILE: Self = Self(b'T');
    pub const CARDBOARD: Self = Self(b'U');
    pub const VENT: Self = Self(b'V');
    pub const WOOD: Self = Self(b'W');
    // do not use - "fake" materials use this (ladders, wading, clips, etc)
    // pub const UNUSED: Self = Self(b'X');
    pub const GLASS: Self = Self(b'Y');
    /// weird-looking jello effect for advisor shield
    pub const WARPSHIELD: Self = Self(b'Z');
}
