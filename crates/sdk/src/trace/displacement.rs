/// Displacement surface flags.
#[repr(C)]
pub struct Displacement(i32);

impl Displacement {
    pub const SURFACE: Self = Self(1);
    pub const WALKABLE: Self = Self(2);
    pub const BUILDABLE: Self = Self(4);
    pub const SURFPROP1: Self = Self(8);
    pub const SURFPROP2: Self = Self(16);
}
