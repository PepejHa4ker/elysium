use super::Networked;

// NOTE: Ensure the indices are synchronized with `insert_entry` in networked.rs

/// `DT_BaseAnimating`
pub struct BaseAnimating<'networked>(pub(super) &'networked Networked);

/// `DT_BaseEntity`
pub struct BaseEntity<'networked>(pub(super) &'networked Networked);

/// `DT_BasePlayer`
pub struct BasePlayer<'networked>(pub(super) &'networked Networked);

/// `DT_BaseCombatWeapon`
pub struct BaseWeapon<'networked>(pub(super) &'networked Networked);

/// `DT_BaseAttributableItem`
pub struct Item<'networked>(pub(super) &'networked Networked);

/// `DT_CSPlayer`
pub struct Player<'networked>(pub(super) &'networked Networked);

/// `DT_WeaponCSBase`
pub struct Weapon<'networked>(pub(super) &'networked Networked);

impl<'networked> BaseAnimating<'networked> {
    /// `m_bClientSideAnimation`
    pub fn client_side_animation(&self) -> usize {
        self.0.base_animating[0]
    }
}

impl<'networked> BaseEntity<'networked> {
    /// `m_nRenderMode`
    pub fn render_mode(&self) -> usize {
        self.0.base_entity[0]
    }
}

impl<'networked> BasePlayer<'networked> {
    /// `deadflag`
    pub fn is_dead(&self) -> usize {
        self.0.base_player[0]
    }

    /// `m_aimPunchAngle`
    pub fn aim_punch_angle(&self) -> usize {
        self.0.base_player[1]
    }

    /// `m_iHealth`
    pub fn health(&self) -> usize {
        self.0.base_player[2]
    }

    /// `m_nTickBase`
    pub fn tick_base(&self) -> usize {
        self.0.base_player[3]
    }

    /// `m_vecVelocity[0]`
    pub fn velocity(&self) -> usize {
        self.0.base_player[4]
    }

    /// `m_vecViewOffset[0]`
    pub fn view_offset(&self) -> usize {
        self.0.base_player[5]
    }

    /// `m_viewPunchAngle`
    pub fn view_punch_angle(&self) -> usize {
        self.0.base_player[6]
    }
}

impl<'networked> BaseWeapon<'networked> {
    /// `m_flNextPrimaryAttack`
    pub fn next_attack_available_after(&self) -> usize {
        self.0.base_weapon[0]
    }

    /// `m_iClip1`
    pub fn ammo(&self) -> usize {
        self.0.base_weapon[1]
    }
}

impl<'networked> Item<'networked> {
    /// `m_iItemDefinitionIndex`
    pub fn index(&self) -> usize {
        self.0.item[0]
    }
}

impl<'networked> Player<'networked> {
    /// `m_ArmorValue`
    pub fn armor(&self) -> usize {
        self.0.player[0]
    }

    /// `m_angEyeAngles[0]`
    pub fn eye_angle(&self) -> usize {
        self.0.player[1]
    }

    /// `m_bGunGameImmunity`
    pub fn is_immune(&self) -> usize {
        self.0.player[2]
    }

    /// `m_bHasDefuser`
    pub fn has_defuse_kit(&self) -> usize {
        self.0.player[3]
    }

    /// `m_bHasHelmet`
    pub fn has_helmet(&self) -> usize {
        self.0.player[4]
    }

    /// `m_bIsScoped`
    pub fn is_scoped(&self) -> usize {
        self.0.player[5]
    }

    /// `m_fFlags`
    pub fn flags(&self) -> usize {
        self.0.player[6]
    }

    /// `m_flLowerBodyYawTarget`
    pub fn lower_body_yaw_target(&self) -> usize {
        self.0.player[7]
    }

    /// `m_hActiveWeapon`
    pub fn weapon(&self) -> usize {
        self.0.player[8]
    }

    /// `m_hObserverTarget`
    pub fn observer_target(&self) -> usize {
        self.0.player[9]
    }

    /// `m_iAccount`
    pub fn money(&self) -> usize {
        self.0.player[10]
    }
}

impl<'networked> Weapon<'networked> {
    /// `m_flPostponeFireReadyTime`
    pub fn revolver_cock_time(&self) -> usize {
        self.0.weapon[0]
    }
}
