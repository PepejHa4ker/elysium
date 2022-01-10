use super::Networked;

// NOTE: Ensure the indices are synchronized with `insert_entry` in networked.rs

pub struct BaseAnimating<'networked>(pub(super) &'networked Networked);
pub struct BaseEntity<'networked>(pub(super) &'networked Networked);
pub struct BasePlayer<'networked>(pub(super) &'networked Networked);
pub struct BaseWeapon<'networked>(pub(super) &'networked Networked);
pub struct Player<'networked>(pub(super) &'networked Networked);
pub struct Weapon<'networked>(pub(super) &'networked Networked);

impl<'networked> BaseAnimating<'networked> {
    pub fn client_side_animation(&self) -> usize {
        self.0.base_animating[0]
    }
}

impl<'networked> BaseEntity<'networked> {
    pub fn render_mode(&self) -> usize {
        self.0.base_entity[0]
    }
}

impl<'networked> BasePlayer<'networked> {
    pub fn is_dead(&self) -> usize {
        self.0.base_player[0]
    }

    pub fn aim_punch_angle(&self) -> usize {
        self.0.base_player[1]
    }

    pub fn health(&self) -> usize {
        self.0.base_player[2]
    }

    pub fn tick_base(&self) -> usize {
        self.0.base_player[3]
    }

    pub fn velocity(&self) -> usize {
        self.0.base_player[4]
    }

    pub fn view_offset(&self) -> usize {
        self.0.base_player[5]
    }

    pub fn view_punch_angle(&self) -> usize {
        self.0.base_player[6]
    }
}

impl<'networked> BaseWeapon<'networked> {
    pub fn next_attack_available_after(&self) -> usize {
        self.0.base_weapon[0]
    }

    pub fn ammo(&self) -> usize {
        self.0.base_weapon[1]
    }
}

impl<'networked> Player<'networked> {
    pub fn armor(&self) -> usize {
        self.0.player[0]
    }

    pub fn eye_angle(&self) -> usize {
        self.0.player[1]
    }

    pub fn is_immune(&self) -> usize {
        self.0.player[2]
    }

    pub fn has_defuse_kit(&self) -> usize {
        self.0.player[3]
    }

    pub fn has_helmet(&self) -> usize {
        self.0.player[4]
    }

    pub fn is_scoped(&self) -> usize {
        self.0.player[5]
    }

    pub fn flags(&self) -> usize {
        self.0.player[6]
    }

    pub fn lower_body_yaw_target(&self) -> usize {
        self.0.player[7]
    }

    pub fn weapon(&self) -> usize {
        self.0.player[8]
    }

    pub fn observer_target(&self) -> usize {
        self.0.player[9]
    }

    pub fn money(&self) -> usize {
        self.0.player[10]
    }
}

impl<'networked> Weapon<'networked> {
    pub fn revolver_cock_time(&self) -> usize {
        self.0.weapon[0]
    }
}
