/// Surface flags.
#[repr(C)]
pub struct Surf(i32);

impl Surf {
    /// value will hold the light strength
    pub const LIGHT: Self = Self(0x0001);

    /// don't draw, indicates we should skylight + draw 2d sky but not draw the 3d skybox
    pub const SKY2D: Self = Self(0x0002);

    /// don't draw, but add to skybox
    pub const SKY: Self = Self(0x0004);

    /// turbulent water warp
    pub const WARP: Self = Self(0x0008);

    pub const TRANS: Self = Self(0x0010);

    /// the surface can not have a portal placed on it
    pub const NOPORTAL: Self = Self(0x0020);

    /// FIXME: this is an xbox hack to work around elimination of trigger surfaces, which breaks occluders
    pub const TRIGGER: Self = Self(0x0040);

    /// don't bother referencing the texture
    pub const NODRAW: Self = Self(0x0080);

    /// make a primary bsp splitter
    pub const HINT: Self = Self(0x0100);

    /// completely ignore, allowing non-closed brushes
    pub const SKIP: Self = Self(0x0200);

    /// don't calculate light
    pub const NOLIGHT: Self = Self(0x0400);

    /// calculate three lightmaps for the surface for bumpmapping
    pub const BUMPLIGHT: Self = Self(0x0800);

    /// don't receive shadows
    pub const NOSHADOWS: Self = Self(0x1000);

    /// don't receive decals
    pub const NODECALS: Self = Self(0x2000);

    /// the surface can not have paint placed on it
    pub const NOPAINT: Self = Self(0x2000);

    /// don't subdivide patches on this surface
    pub const NOCHOP: Self = Self(0x4000);

    /// surface is part of a hitbox
    pub const HITBOX: Self = Self(0x8000);
}
