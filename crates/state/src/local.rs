//! Local player-related values.

use super::{Shared, SharedOption, STATE};
use core::ptr::NonNull;
use providence_math::Vec3;
use providence_model::Bones;

pub(crate) struct Local {
    pub player: SharedOption<NonNull<usize>>,
    pub bones: Shared<Bones>,
    pub weapon: SharedOption<NonNull<usize>>,
    pub magazine_ammo: Shared<i32>,
    pub total_ammo: Shared<i32>,
    pub health: Shared<i32>,
    pub view_angle: Shared<Vec3>,
}

impl Local {
    pub const fn new() -> Self {
        Self {
            player: SharedOption::none(),
            bones: Shared::new(Bones::zero()),
            weapon: SharedOption::none(),
            magazine_ammo: Shared::new(0),
            total_ammo: Shared::new(0),
            health: Shared::new(0),
            view_angle: Shared::new(Vec3::splat(0.0)),
        }
    }
}

/// Return's a reference to the local player's bones.
#[inline]
pub unsafe fn bones() -> &'static mut Bones {
    STATE.local.bones.as_mut()
}

/// Return's a reference to the local player's view_angle.
#[inline]
pub unsafe fn view_angle() -> &'static mut Vec3 {
    STATE.local.view_angle.as_mut()
}
