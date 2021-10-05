#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PlayerState(i32);

impl PlayerState {
    /// On the ground.
    pub const ON_GROUND: Self = Self::new(1 << 0);

    /// Player is fully crouched.
    pub const DUCKING: Self = Self::new(1 << 1);

    /// Jumping out of water.
    pub const WATER_JUMP: Self = Self::new(1 << 2);

    /// Player is controlling a train. Movement commands should be ignored during prediction.
    pub const ON_TRAIN: Self = Self::new(1 << 3);

    /// Player is standing in rain.
    pub const IN_RAIN: Self = Self::new(1 << 4);

    /// Player is frozen for 3rd person camera.
    pub const FROZEN: Self = Self::new(1 << 5);

    /// Player can't move. Controlling another entity.
    pub const AT_CONTROLS: Self = Self::new(1 << 6);

    /// Is a player.
    pub const CLIENT: Self = Self::new(1 << 7);

    /// Fake client, simulated server side. Don't send network messages to them.
    pub const FAKE_CLIENT: Self = Self::new(1 << 8);

    /// Partially on the ground.
    pub const PARTIAL_ON_GROUND: Self = Self::new(1 << 18);

    /// Entity is in water.
    pub const IN_WATER: Self = Self::new(1 << 9);

    const fn new(state: i32) -> Self {
        Self(state)
    }

    pub const fn on_ground(&self) -> bool {
        self.0 & Self::ON_GROUND.0 != 0
    }

    pub const fn is_ducking(&self) -> bool {
        self.0 & Self::DUCKING.0 != 0
    }

    pub const fn doing_water_jump(&self) -> bool {
        self.0 & Self::WATER_JUMP.0 != 0
    }

    pub const fn on_train(&self) -> bool {
        self.0 & Self::ON_TRAIN.0 != 0
    }

    pub const fn in_rain(&self) -> bool {
        self.0 & Self::IN_RAIN.0 != 0
    }

    pub const fn is_frozen(&self) -> bool {
        self.0 & Self::FROZEN.0 != 0
    }

    pub const fn at_controls(&self) -> bool {
        self.0 & Self::AT_CONTROLS.0 != 0
    }

    pub const fn is_client(&self) -> bool {
        self.0 & Self::CLIENT.0 != 0
    }

    pub const fn is_fake_client(&self) -> bool {
        self.0 & Self::FAKE_CLIENT.0 != 0
    }

    pub const fn partially_on_ground(&self) -> bool {
        self.0 & Self::PARTIAL_ON_GROUND.0 != 0
    }

    pub const fn in_water(&self) -> bool {
        self.0 & Self::IN_WATER.0 != 0
    }
}
