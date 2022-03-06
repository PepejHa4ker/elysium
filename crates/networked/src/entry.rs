/// Networked variable name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Entry {
    /// `m_aimPunchAngle`
    AimPunchAngle,

    /// `m_iClip1`
    Ammo,

    /// `m_ArmorValue`
    Armor,

    /// `m_bClientSideAnimation`
    ClientSideAnimation,

    /// `m_angEyeAngles[0]`
    EyeAngle,

    /// `m_fFlags`
    Flags,

    /// `m_bHasDefuser`
    HasDefuseKit,

    /// `m_bHasHelmet`
    HasHelmet,

    /// `m_iHealth`
    Health,

    /// `m_iItemDefinitionIndex`
    Index,

    /// `m_bInReload`
    InReload,

    /// `deadflag`
    IsDead,

    /// `m_bGunGameImmunity`
    IsImmune,

    /// `m_bIsScoped`
    IsScoped,

    /// `m_iAccount`
    Money,

    /// `m_flNextPrimaryAttack`
    NextAttackAvailableAfter,

    /// `m_flLowerBodyYawTarget`
    LowerBodyYawTarget,

    /// `m_hObserverTarget`
    ObserverTarget,

    /// `m_nRenderMode`
    RenderMode,

    /// `m_flPostponeFireReadyTime`
    RevolverCockTime,

    /// `m_nTickBase`
    TickBase,

    /// `m_vecVelocity[0]`
    Velocity,

    /// `m_vecViewOffset[0]`
    ViewOffset,

    /// `m_viewPunchAngle`
    ViewPunchAngle,

    /// `m_hActiveWeapon`
    Weapon,
}

static ENTRIES: phf::Map<&'static str, Entry> = phf::phf_map! {
    "m_aimPunchAngle" => Entry::AimPunchAngle,
    "m_ArmorValue" => Entry::Armor,
    "m_iClip1" => Entry::Ammo,
    "m_bClientSideAnimation" => Entry::ClientSideAnimation,
    "m_angEyeAngles[0]" => Entry::EyeAngle,
    "m_fFlags" => Entry::Flags,
    "m_bHasDefuser" => Entry::HasDefuseKit,
    "m_iHealth" => Entry::Health,
    "m_iItemDefinitionIndex" => Entry::Index,
    "m_bInReload" => Entry::InReload,
    "deadflag" => Entry::IsDead,
    "m_bHasHelmet" => Entry::HasHelmet,
    "m_bGunGameImmunity" => Entry::IsImmune,
    "m_bIsScoped" => Entry::IsScoped,
    "m_iAccount" => Entry::Money,
    "m_flNextPrimaryAttack" => Entry::NextAttackAvailableAfter,
    "m_flLowerBodyYawTarget" => Entry::LowerBodyYawTarget,
    "m_hObserverTarget" => Entry::ObserverTarget,
    "m_nRenderMode" => Entry::RenderMode,
    "m_flPostponeFireReadyTime" => Entry::RevolverCockTime,
    "m_nTickBase" => Entry::TickBase,
    "m_vecVelocity[0]" => Entry::Velocity,
    "m_vecViewOffset[0]" => Entry::ViewOffset,
    "m_viewPunchAngle" => Entry::ViewPunchAngle,
    "m_hActiveWeapon" => Entry::Weapon,
};

impl Entry {
    /// Map a string into an entry we're interested in.
    pub fn from_str(entry: &str) -> Option<Self> {
        ENTRIES.get(entry).cloned()
    }

    /// Returns the value this entry maps to.
    pub fn as_str(&self) -> &'static str {
        match self {
            Entry::AimPunchAngle => "m_aimPunchAngle",
            Entry::Armor => "m_ArmorValue",
            Entry::Ammo => "m_iClip1",
            Entry::ClientSideAnimation => "m_bClientSideAnimation",
            Entry::EyeAngle => "m_angEyeAngles[0]",
            Entry::Flags => "m_fFlags",
            Entry::HasDefuseKit => "m_bHasDefuser",
            Entry::HasHelmet => "m_bHasHelmet",
            Entry::Health => "m_iHealth",
            Entry::Index => "m_iItemDefinitionIndex",
            Entry::InReload => "m_bInReload",
            Entry::IsDead => "deadflag",
            Entry::IsImmune => "m_bGunGameImmunity",
            Entry::IsScoped => "m_bIsScoped",
            Entry::Money => "m_iAccount",
            Entry::NextAttackAvailableAfter => "m_flNextPrimaryAttack",
            Entry::LowerBodyYawTarget => "m_flLowerBodyYawTarget",
            Entry::ObserverTarget => "m_hObserverTarget",
            Entry::RenderMode => "m_nRenderMode",
            Entry::RevolverCockTime => "m_flPostponeFireReadyTime",
            Entry::TickBase => "m_nTickBase",
            Entry::Velocity => "m_vecVelocity[0]",
            Entry::ViewOffset => "m_vecViewOffset[0]",
            Entry::ViewPunchAngle => "m_viewPunchAngle",
            Entry::Weapon => "m_hActiveWeapon",
            _ => "(none)",
        }
    }
}
