use std::ops::BitAnd;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct State(u32);

impl State {
    pub const ATTACK: Self = Self::new(1 << 0);
    pub const JUMP: Self = Self::new(1 << 1);
    pub const DUCK: Self = Self::new(1 << 2);
    pub const FORWARD: Self = Self::new(1 << 3);
    pub const BACK: Self = Self::new(1 << 4);
    pub const USE: Self = Self::new(1 << 5);
    pub const CANCEL: Self = Self::new(1 << 6);
    pub const LEFT: Self = Self::new(1 << 7);
    pub const RIGHT: Self = Self::new(1 << 8);
    pub const MOVELEFT: Self = Self::new(1 << 9);
    pub const MOVERIGHT: Self = Self::new(1 << 10);
    pub const ATTACK2: Self = Self::new(1 << 11);
    pub const RUN: Self = Self::new(1 << 12);
    pub const RELOAD: Self = Self::new(1 << 13);
    pub const ALT1: Self = Self::new(1 << 14);
    pub const ALT2: Self = Self::new(1 << 15);
    pub const SCORE: Self = Self::new(1 << 16);
    pub const SPEED: Self = Self::new(1 << 17);
    pub const WALK: Self = Self::new(1 << 18);
    pub const ZOOM: Self = Self::new(1 << 19);
    pub const WEAPON1: Self = Self::new(1 << 20);
    pub const WEAPON2: Self = Self::new(1 << 21);
    pub const BULLRUSH: Self = Self::new(1 << 22);
    pub const GRENADE1: Self = Self::new(1 << 23);
    pub const GRENADE2: Self = Self::new(1 << 24);
    pub const ATTACK3: Self = Self::new(1 << 25);

    const fn new(state: u32) -> Self {
        Self(state)
    }

    pub const fn is_attack(&self) -> bool {
        self.0 & Self::ATTACK.0 == 0
    }

    pub const fn is_attack2(&self) -> bool {
        self.0 & Self::ATTACK2.0 == 0
    }

    pub const fn is_attack3(&self) -> bool {
        self.0 & Self::ATTACK3.0 == 0
    }
}
