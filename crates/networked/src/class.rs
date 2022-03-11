macro_rules! classes {
    ($($name:ident => $string:literal),*) => {
        /// Networked class name.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum Class {
            $(
                $name,
            )*
        }

        static CLASSES: phf::Map<&'static str, Class> = phf::phf_map! {
            $(
                $string => Class::$name,
            )*
        };

        impl Class {
            /// Map a string into an entry we're interested in.
            pub fn from_str(entry: &str) -> Option<Self> {
                CLASSES.get(entry).cloned()
            }

            /// Returns the value this entry maps to.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Class::$name => $string,
                    )*
                }
            }
        }
    };
}

classes! {
    BaseAnimating => "DT_BaseAnimating",
    BaseEntity => "DT_BaseEntity",
    BasePlayer => "DT_BasePlayer",
    BaseWeapon => "DT_BaseCombatWeapon",
    Bomb => "DT_PlantedC4",
    Fog => "DT_FogController",
    Item => "DT_BaseAttributableItem",
    Player => "DT_CSPlayer",
    Tomemap => "DT_EnvTonemapController",
    Weapon => "DT_WeaponCSBase"
}
