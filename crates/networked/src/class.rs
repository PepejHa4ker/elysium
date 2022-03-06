/// Networked class name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Class {
    /// `DT_BaseAnimating`
    BaseAnimating,

    /// `DT_BaseEntity`
    BaseEntity,

    /// `DT_BasePlayer`
    BasePlayer,

    /// `DT_BaseCombatWeapon`
    BaseWeapon,

    /// `DT_PlantedC4`
    Bomb,

    /// `DT_FogController`
    Fog,

    /// `DT_BaseAttributableItem`
    Item,

    /// `DT_CSPlayer`
    Player,

    /// `DT_EnvTonemapController`
    Tomemap,

    /// `DT_WeaponCSBase`
    Weapon,
}

static CLASSES: phf::Map<&'static str, Class> = phf::phf_map! {
    "DT_BaseAnimating" => Class::BaseAnimating,
    "DT_BaseEntity" => Class::BaseEntity,
    "DT_BasePlayer" => Class::BasePlayer,
    "DT_BaseCombatWeapon" => Class::BaseWeapon,
    "DT_BaseAttributableItem" => Class::Item,
    "DT_CSPlayer" => Class::Player,
    "DT_EnvTonemapController" => Class::Tomemap,
    "DT_PlantedC4" => Class::Bomb,
    "DT_FogController" => Class::Fog,
    "DT_WeaponCSBase" => Class::Weapon,
};

impl Class {
    /// Map a string into a class we're interested in.
    pub fn from_str(class: &str) -> Option<Self> {
        CLASSES.get(class).cloned()
    }

    /// Returns the value this class maps to.
    pub fn as_str(&self) -> &'static str {
        match self {
            Class::BaseAnimating => "DT_BaseAnimating",
            Class::BaseEntity => "DT_BaseEntity",
            Class::BasePlayer => "DT_BasePlayer",
            Class::BaseWeapon => "DT_BaseCombatWeapon",
            Class::Bomb => "DT_PlantedC4",
            Class::Fog => "DT_FogController",
            Class::Item => "DT_BaseAttributableItem",
            Class::Player => "DT_CSPlayer",
            Class::Tomemap => "DT_EnvTonemapController",
            Class::Weapon => "DT_WeaponCSBase",
        }
    }
}
