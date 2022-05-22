pub mod filter;

pub mod ray {
    #[repr(C)]
    struct VTable {}

    #[repr(C)]
    pub struct Ray {
        vtable: &'static VTable,
    }
}

pub trait Filter {
    fn should_hit_entity(&self, entity: *const (), mask: i32) -> bool;
    fn get_trace_kind(&self) -> TraceKind;
}

#[repr(C)]
struct VTable {}

#[repr(C)]
pub struct Trace {
    vtable: &'static VTable,
}

pub mod surf {
    #[repr(C)]
    pub struct Surf(i32);

    impl Surf {
        /// value will hold the light strength
        pub const SURF_LIGHT: Self = Self(0x0001);

        /// don't draw, indicates we should skylight + draw 2d sky but not draw the 3d skybox
        pub const SURF_SKY2D: Self = Self(0x0002);

        /// don't draw, but add to skybox
        pub const SURF_SKY: Self = Self(0x0004);

        /// turbulent water warp
        pub const SURF_WARP: Self = Self(0x0008);

        pub const SURF_TRANS: Self = Self(0x0010);

        /// the surface can not have a portal placed on it
        pub const SURF_NOPORTAL: Self = Self(0x0020);

        /// FIXME: this is an xbox hack to work around elimination of trigger surfaces, which breaks occluders
        pub const SURF_TRIGGER: Self = Self(0x0040);

        /// don't bother referencing the texture
        pub const SURF_NODRAW: Self = Self(0x0080);

        /// make a primary bsp splitter
        pub const SURF_HINT: Self = Self(0x0100);

        /// completely ignore, allowing non-closed brushes
        pub const SURF_SKIP: Self = Self(0x0200);

        /// don't calculate light
        pub const SURF_NOLIGHT: Self = Self(0x0400);

        /// calculate three lightmaps for the surface for bumpmapping
        pub const SURF_BUMPLIGHT: Self = Self(0x0800);

        /// don't receive shadows
        pub const SURF_NOSHADOWS: Self = Self(0x1000);

        /// don't receive decals
        pub const SURF_NODECALS: Self = Self(0x2000);

        /// the surface can not have paint placed on it
        pub const SURF_NOPAINT: Self = Self(0x2000);

        /// don't subdivide patches on this surface
        pub const SURF_NOCHOP: Self = Self(0x4000);

        /// surface is part of a hitbox
        pub const SURF_HITBOX: Self = Self(0x8000);
    }

    #[repr(C)]
    pub struct Tex(u8);

    impl Tex {
        pub const CHAR_TEX_ANTLION: Self = Self(b'A');
        pub const CHAR_TEX_BLOODYFLESH: Self = Self(b'B');
        pub const CHAR_TEX_CONCRETE: Self = Self(b'C');
        pub const CHAR_TEX_DIRT: Self = Self(b'D');
        /// the egg sacs in the tunnels in ep2
        pub const CHAR_TEX_EGGSHELL: Self = Self(b'E');
        pub const CHAR_TEX_FLESH: Self = Self(b'F');
        pub const CHAR_TEX_GRATE: Self = Self(b'G');
        pub const CHAR_TEX_ALIENFLESH: Self = Self(b'H');
        pub const CHAR_TEX_CLIP: Self = Self(b'I');
        // pub const CHAR_TEX_UNUSED: Self = Self(b'J');
        pub const CHAR_TEX_SNOW: Self = Self(b'K');
        pub const CHAR_TEX_PLASTIC: Self = Self(b'L');
        pub const CHAR_TEX_METAL: Self = Self(b'M');
        pub const CHAR_TEX_SAND: Self = Self(b'N');
        pub const CHAR_TEX_FOLIAGE: Self = Self(b'O');
        pub const CHAR_TEX_COMPUTER: Self = Self(b'P');
        // pub const CHAR_TEX_UNUSED: Self = Self(b'Q');
        pub const CHAR_TEX_REFLECTIVE: Self = Self(b'R');
        pub const CHAR_TEX_SLOSH: Self = Self(b'S');
        pub const CHAR_TEX_TILE: Self = Self(b'T');
        pub const CHAR_TEX_CARDBOARD: Self = Self(b'U');
        pub const CHAR_TEX_VENT: Self = Self(b'V');
        pub const CHAR_TEX_WOOD: Self = Self(b'W');
        // do not use - "fake" materials use this (ladders, wading, clips, etc)
        // pub const CHAR_TEX_UNUSED: Self = Self(b'X');
        pub const CHAR_TEX_GLASS: Self = Self(b'Y');
        /// weird-looking jello effect for advisor shield
        pub const CHAR_TEX_WARPSHIELD: Self = Self(b'Z');
    }

    #[repr(C)]
    pub struct Contents(u32);

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

        /// ignore OPAQUE on surfaces that have SURF_NODRAW
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

    #[repr(C)]
    pub struct Mask(u32);

    impl Mask {
        pub const ALL: Self = Self(0xFFFFFFFF);

        /// everything that is normally solid
        pub const SOLID: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::WINDOW.0
                | Contents::MONSTER.0
                | Contents::GRATE.0,
        );

        /// everything that blocks player movement
        pub const PLAYERSOLID: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::PLAYERCLIP.0
                | Contents::WINDOW.0
                | Contents::MONSTER.0
                | Contents::GRATE.0,
        );

        /// blocks npc movement
        pub const NPCSOLID: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::MONSTERCLIP.0
                | Contents::WINDOW.0
                | Contents::MONSTER.0
                | Contents::GRATE.0,
        );

        /// water physics in these contents
        pub const WATER: Self = Self(Contents::WATER.0 | Contents::MOVEABLE.0 | Contents::SLIME.0);

        /// everything that blocks line of sight for ai, lighting, etc
        pub const OPAQUE: Self =
            Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::OPAQUE.0);

        /// everything that blocks line of sight for ai, lighting, etc, but with monsters added
        pub const OPAQUE_AND_NPCS: Self = Self(Self::OPAQUE.0 | Contents::MONSTER.0);

        /// everything that blocks line of sight for players
        pub const VISIBLE: Self = Self(Self::OPAQUE.0 | Contents::IGNORE_NODRAW_OPAQUE.0);

        /// everything that blocks line of sight for players, but with monsters added
        pub const VISIBLE_AND_NPCS: Self =
            Self(Self::OPAQUE_AND_NPCS.0 | Contents::IGNORE_NODRAW_OPAQUE.0);

        /// bullets see these as solid
        pub const SHOT: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::MONSTER.0
                | Contents::WINDOW.0
                | Contents::DEBRIS.0
                | Contents::HITBOX.0,
        );

        /// non-raycasted weapons see this as solid (includes grates)
        pub const SHOT_HULL: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::MONSTER.0
                | Contents::WINDOW.0
                | Contents::DEBRIS.0
                | Contents::GRATE.0,
        );

        /// hits solids (not grates) and passes through everything else
        pub const SHOT_PORTAL: Self =
            Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::WINDOW.0);

        /// everything normally solid, except monsters (world+brush only)
        pub const SOLID_BRUSHONLY: Self =
            Self(Contents::SOLID.0 | Contents::MOVEABLE.0 | Contents::WINDOW.0 | Contents::GRATE.0);

        /// everything normally solid for player movement, except monsters (world+brush only)
        pub const PLAYERSOLID_BRUSHONLY: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::WINDOW.0
                | Contents::PLAYERCLIP.0
                | Contents::GRATE.0,
        );

        /// everything normally solid for npc movement, except monsters (world+brush only)
        pub const NPCSOLID_BRUSHONLY: Self = Self(
            Contents::SOLID.0
                | Contents::MOVEABLE.0
                | Contents::WINDOW.0
                | Contents::MONSTERCLIP.0
                | Contents::GRATE.0,
        );

        /// just the world, used for route rebuilding
        pub const NPCWORLDSTATIC: Self = Self(
            Contents::SOLID.0 | Contents::WINDOW.0 | Contents::MONSTERCLIP.0 | Contents::GRATE.0,
        );

        /// These are things that can split areaportals
        pub const SPLITAREAPORTAL: Self = Self(Contents::WATER.0 | Contents::SLIME.0);
    }
}

#[repr(i32)]
pub enum TraceKind {
    Everything = 0,
    /// this does not test static props
    WorldOnly = 1,
    /// this version will not test static props
    EntitiesOnly = 2,
    /// everything filter props
    EverythingFilterProps = 3,
}
