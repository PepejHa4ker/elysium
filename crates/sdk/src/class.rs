macro_rules! classes {
    ($($name:ident => $string:literal),*) => {
        /// networked class name
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum Class {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                #[doc(alias = $string)]
                $name,
            )*
        }

        const CLASSES: phf::Map<&'static str, Class> = phf::phf_map! {
            $(
                $string => Class::$name,
            )*
        };

        impl Class {
            /// map a string into an entry we're interested in
            #[inline]
            pub fn from_str(entry: &str) -> Option<Self> {
                CLASSES.get(entry).cloned()
            }

            /// returns this class as a string
            #[inline]
            pub const fn as_str(&self) -> &'static str {
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
