const EMPTY: u32 = 0; // No contents

const SOLID: u32 = 0x1; // an eye is never valid in a solid
const WINDOW: u32 = 0x2; // translucent, but not watery: u32 = glass;
const AUX: u32 = 0x4;
const GRATE: u32 = 0x8; // alpha-tested "grate" textures. Bullets/sight pass through, but solids don't
const SLIME: u32 = 0x10;
const WATER: u32 = 0x20;
const BLOCKLOS: u32 = 0x40; // block AI line of sight
const OPAQUE: u32 = 0x80; // things that cannot be seen through: u32 = may be non-solid though;

const TESTFOGVOLUME: u32 = 0x100;
const UNUSED: u32 = 0x200;

// unused
// NOTE: If it's visible, grab from the top update LAST_VISIBLE_CONTENTS
// if not visible, then grab from the bottom.
// OPAQUE SURF_NODRAW count as OPAQUE: u32 = shadow-casting toolsblocklight textures;
const BLOCKLIGHT: u32 = 0x400;

const TEAM1: u32 = 0x800; // per team contents used to differentiate collisions
const TEAM2: u32 = 0x1000; // between players and objects on different teams

// ignore OPAQUE on surfaces that have SURF_NODRAW
const IGNORE_NODRAW_OPAQUE: u32 = 0x2000;

// hits entities which are MOVETYPE_PUSH: u32 = doors, plats, etc.;
const MOVEABLE: u32 = 0x4000;

// remaining contents are non-visible, and don't eat brushes
const AREAPORTAL: u32 = 0x8000;

const PLAYERCLIP: u32 = 0x10000;
const MONSTERCLIP: u32 = 0x20000;

// currents can be added to any other contents, and may be mixed
const CURRENT_0: u32 = 0x40000;
const CURRENT_90: u32 = 0x80000;
const CURRENT_180: u32 = 0x100000;
const CURRENT_270: u32 = 0x200000;
const CURRENT_UP: u32 = 0x400000;
const CURRENT_DOWN: u32 = 0x800000;

const ORIGIN: u32 = 0x1000000; // removed before bsping an entity

const MONSTER: u32 = 0x2000000; // should never be on a brush, only in game
const DEBRIS: u32 = 0x4000000;

// brushes to be added after vis leafs
const DETAIL: u32 = 0x8000000;

// auto set if any surface has trans
const TRANSLUCENT: u32 = 0x10000000;
const LADDER: u32 = 0x20000000;

// use accurate hitboxes on trace
const HITBOX: u32 = 0x40000000;

macro_rules! flag {
    { $flag:ident, $set:ident, $has:ident } => {
        pub const fn $set(self) -> Self {
            self.with($flag)
        }

        pub const fn $has(&self) -> bool {
            self.has($flag)
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct Contents(u32);

impl Contents {
    pub const fn new() -> Self {
        Self(0)
    }

    const fn with(mut self, flag: u32) -> Self {
        self.0 |= flag;
        self
    }

    const fn has(&self, flag: u32) -> bool {
        (self.0 & flag) != 0
    }

    pub const fn to_u32(&self) -> u32 {
        self.0
    }

    pub const fn to_u16(&self) -> u16 {
        self.0 as u16
    }

    flag! { DEBRIS, debris, has_debris }
    flag! { GRATE, grate, has_grate }
    flag! { HITBOX, hitbox, has_hitbox }
    flag! { MONSTER, monster, has_monster }
    flag! { MOVEABLE, moveable, has_moveable }
    flag! { SOLID, solid, has_solid }
    flag! { WINDOW, window, has_window }
}
