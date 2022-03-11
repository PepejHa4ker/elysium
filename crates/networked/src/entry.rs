macro_rules! entries {
    ($($name:ident => $string:literal),*) => {
        /// Networked variable name.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum Entry {
            $(
                $name,
            )*
        }

        static ENTRIES: phf::Map<&'static str, Entry> = phf::phf_map! {
            $(
                $string => Entry::$name,
            )*
        };

        impl Entry {
            /// Map a string into an entry we're interested in.
            pub fn from_str(entry: &str) -> Option<Self> {
                ENTRIES.get(entry).cloned()
            }

            /// Returns the value this entry maps to.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Entry::$name => $string,
                    )*
                }
            }
        }
    };
}

entries! {
    AccuracyPenalty => "m_fAccuracyPenalty",
    AimPunchAngle => "m_aimPunchAngle",
    AnimationTime => "m_flAnimTime",
    Armor => "m_ArmorValue",

    BloomScale => "m_flCustomBloomScale",

    ClientSideAnimation => "m_bClientSideAnimation",
    Collision => "m_Collision",
    Cycle => "m_flCycle",

    EnableBloomScale => "m_bUseCustomBloomScale",
    EnableExposureMax => "m_bUseCustomAutoExposureMax",
    EnableExposureMin => "m_bUseCustomAutoExposureMin",
    ExposureMax => "m_flCustomAutoExposureMax",
    ExposureMin => "m_flCustomAutoExposureMin",
    EyeAngle => "m_angEyeAngles[0]",

    Flags => "m_fFlags",
    FogColorPrimary => "m_fog.colorPrimary",
    FogColorSecondary => "m_fog.colorSecondary",
    FogDensity => "m_fog.maxdensity",
    FogEnd => "m_fog.end",
    FogFarZ => "m_fog.farz",
    FogIsEnabled => "m_fog.enable",
    FogHDRScale => "m_fog.HDRColorScale",
    FogStart => "m_fog.start",
    Frozen => "m_flFrozen",

    HasDefuseKit => "m_bHasDefuser",
    HasHelmet => "m_bHasHelmet",
    Health => "m_iHealth",

    InReload => "m_bInReload",
    IsDead => "deadflag",
    IsDefused => "m_mBombDefused",
    IsImmune => "m_bGunGameImmunity",
    IsScoped => "m_bIsScoped",
    ItemIndex => "m_iItemDefinitionIndex",

    LowerBodyYaw => "m_flLowerBodyYawTarget",

    Magazine => "m_iClip1",
    MaxFlashAlpha => "m_flFlashMaxAlpha",
    Money => "m_iAccount",

    NextAttackAvailableAfter => "m_flNextPrimaryAttack",

    Observer => "m_hObserverTarget",

    PoseParameter => "m_flPoseParameter",
    RenderMode => "m_nRenderMode",

    RevolverCockTime => "m_flPostponeFireReadyTime",

    Sequence => "m_nSequence",
    SimulationTime => "m_flSimulationTime",
    Skin => "m_nSkin",
    SurvivalTeam => "m_nSurvivalTeam",

    Team => "m_iTeamNum",
    TickBase => "m_nTickBase",
    TimeRemaining => "m_flC4Blow",

    Velocity => "m_vecVelocity[0]",
    ViewOffset => "m_vecViewOffset[0]",
    ViewPunchAngle => "m_viewPunchAngle",

    Weapon => "m_hActiveWeapon"
}
