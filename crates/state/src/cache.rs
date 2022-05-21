//! In-game entity cache.

use core::ops::{Deref, DerefMut};
use providence_model::Bones;

/// A cached player.
pub struct Player {
    pub bones: Bones,
    pub id: u64,
    pub index: i32,
    pub health: i32,
    pub magazine_ammo: i32,
    pub name: String,
    pub next_attack_available_after: f32,
    pub revolver_cock_time: f32,
    pub total_ammo: i32,
}

impl Player {
    pub const fn new() -> Player {
        let bones = Bones::zero();
        let id = 0;
        let index = 0;
        let health = 0;
        let magazine_ammo = 0;
        let name = String::new();
        let next_attack_available_after = 0.0;
        let revolver_cock_time = 0.0;
        let total_ammo = 0;

        Self {
            bones,
            health,
            id,
            index,
            magazine_ammo,
            name,
            next_attack_available_after,
            revolver_cock_time,
            total_ammo,
        }
    }

    pub fn name(&self) -> &str {
        &*self.name
    }
}

/// An array of cached players.
pub struct Players {
    players: [Player; 64],
}

impl Players {
    const INIT: Player = Player::new();

    pub const fn new() -> Players {
        let players = [Self::INIT; 64];

        Self { players }
    }
}

impl Deref for Players {
    type Target = [Player; 64];

    fn deref(&self) -> &Self::Target {
        &self.players
    }
}

impl DerefMut for Players {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.players
    }
}
