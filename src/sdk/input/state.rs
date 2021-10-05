#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct State(pub i32);

impl State {
    pub const ATTACK: Self = Self::new(1 << 0);
    pub const JUMP: Self = Self::new(1 << 1);
    pub const CROUCH: Self = Self::new(1 << 2);
    pub const FORWARD: Self = Self::new(1 << 3);
    pub const BACKWARD: Self = Self::new(1 << 4);
    pub const USE: Self = Self::new(1 << 5);
    pub const CANCEL: Self = Self::new(1 << 6);
    pub const LEFT: Self = Self::new(1 << 7);
    pub const RIGHT: Self = Self::new(1 << 8);
    pub const MOVE_LEFT: Self = Self::new(1 << 9);
    pub const MOVE_RIGHT: Self = Self::new(1 << 10);
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

    const fn new(state: i32) -> Self {
        Self(state)
    }

    pub const fn is_attack(&self) -> bool {
        self.0 & Self::ATTACK.0 != 0
    }

    pub const fn is_attack2(&self) -> bool {
        self.0 & Self::ATTACK2.0 != 0
    }

    pub const fn is_attack3(&self) -> bool {
        self.0 & Self::ATTACK3.0 != 0
    }

    pub const fn is_any_attack(&self) -> bool {
        self.0 & (Self::ATTACK.0 | Self::ATTACK2.0 | Self::ATTACK3.0) != 0
    }

    pub const fn is_jump(&self) -> bool {
        self.0 & Self::JUMP.0 != 0
    }

    pub const fn is_crouch(&self) -> bool {
        self.0 & Self::CROUCH.0 != 0
    }

    pub const fn is_forward(&self) -> bool {
        self.0 & Self::FORWARD.0 != 0
    }

    pub const fn is_backward(&self) -> bool {
        self.0 & Self::BACKWARD.0 != 0
    }

    pub const fn is_use(&self) -> bool {
        self.0 & Self::USE.0 != 0
    }

    pub const fn is_cancel(&self) -> bool {
        self.0 & Self::CANCEL.0 != 0
    }

    pub const fn is_left(&self) -> bool {
        self.0 & Self::LEFT.0 != 0
    }

    pub const fn is_right(&self) -> bool {
        self.0 & Self::RIGHT.0 != 0
    }

    pub const fn is_move_right(&self) -> bool {
        self.0 & Self::MOVE_RIGHT.0 != 0
    }

    pub const fn is_move_left(&self) -> bool {
        self.0 & Self::MOVE_LEFT.0 != 0
    }

    pub const fn is_run(&self) -> bool {
        self.0 & Self::RUN.0 != 0
    }

    pub const fn is_reload(&self) -> bool {
        self.0 & Self::RELOAD.0 != 0
    }

    pub const fn is_alt1(&self) -> bool {
        self.0 & Self::ALT1.0 != 0
    }

    pub const fn is_alt2(&self) -> bool {
        self.0 & Self::ALT2.0 != 0
    }

    pub const fn is_score(&self) -> bool {
        self.0 & Self::SCORE.0 != 0
    }

    pub const fn is_speed(&self) -> bool {
        self.0 & Self::SPEED.0 != 0
    }

    pub const fn is_walk(&self) -> bool {
        self.0 & Self::WALK.0 != 0
    }

    pub const fn is_zoom(&self) -> bool {
        self.0 & Self::ZOOM.0 != 0
    }

    pub const fn is_weapon1(&self) -> bool {
        self.0 & Self::WEAPON1.0 != 0
    }

    pub const fn is_weapon2(&self) -> bool {
        self.0 & Self::WEAPON2.0 != 0
    }

    pub const fn is_bullrush(&self) -> bool {
        self.0 & Self::BULLRUSH.0 != 0
    }

    pub const fn is_grenade1(&self) -> bool {
        self.0 & Self::GRENADE1.0 != 0
    }

    pub const fn is_grenade2(&self) -> bool {
        self.0 & Self::GRENADE2.0 != 0
    }
}
