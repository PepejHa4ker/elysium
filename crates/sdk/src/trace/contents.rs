/// Contents for a trace.
#[repr(C)]
pub struct Contents(pub u32);

impl Contents {
    /// no contents
    pub const EMPTY: Self = Self(0);

    /// an eye is never valid in a solid
    pub const SOLID: Self = Self(0x1);

    /// translucent, but not watery (glass)
    pub const WINDOW: Self = Self(0x2);

    pub const AUX: Self = Self(0x4);

    /// alpha-tested "grate" textures. bullets/sight pass through, but solids don't
    pub const GRATE: Self = Self(0x8);

    pub const SLIME: Self = Self(0x10);

    pub const WATER: Self = Self(0x20);

    pub const MIST: Self = Self(0x40);

    /// things that cannot be seen through (may be non-solid though)
    pub const OPAQUE: Self = Self(0x80);

    pub const LAST_VISIBLE_CONTENTS: Self = Self(0x80);

    pub const ALL_VISIBLE_CONTENTS: Self =
        Self(Self::LAST_VISIBLE_CONTENTS.0 | (Self::LAST_VISIBLE_CONTENTS.0 - 1));

    pub const TESTFOGVOLUME: Self = Self(0x100);

    pub const UNUSED5: Self = Self(0x200);

    pub const UNUSED6: Self = Self(0x4000);

    /// per team contents used to differentiate collisions
    pub const TEAM1: Self = Self(0x800);

    /// between players and objects on different teams
    pub const TEAM2: Self = Self(0x1000);

    /// ignore OPAQUE on surfaces that have NODRAW
    pub const IGNORE_NODRAW_OPAQUE: Self = Self(0x2000);

    /// hits entities which are MOVETYPE_PUSH (doors, plats, etc)
    pub const MOVEABLE: Self = Self(0x4000);

    /// remaining contents are non-visible, and don't eat brushes
    pub const AREAPORTAL: Self = Self(0x8000);

    pub const PLAYERCLIP: Self = Self(0x10000);

    pub const MONSTERCLIP: Self = Self(0x20000);

    pub const CURRENT_0: Self = Self(0x40000);
    pub const CURRENT_90: Self = Self(0x80000);
    pub const CURRENT_180: Self = Self(0x100000);
    pub const CURRENT_270: Self = Self(0x200000);
    pub const CURRENT_UP: Self = Self(0x400000);
    pub const CURRENT_DOWN: Self = Self(0x800000);

    /// removed before bsping an entity
    pub const ORIGIN: Self = Self(0x1000000);
    /// should never be on a brush, only in game
    pub const MONSTER: Self = Self(0x2000000);
    pub const DEBRIS: Self = Self(0x4000000);
    /// brushes to be added after vis leafs
    pub const DETAIL: Self = Self(0x8000000);
    /// auto set if any surface has trans
    pub const TRANSLUCENT: Self = Self(0x10000000);
    pub const LADDER: Self = Self(0x20000000);
    /// use accurate hitboxes on trace
    pub const HITBOX: Self = Self(0x40000000);
}
