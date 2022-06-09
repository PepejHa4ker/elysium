#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(i32)]
pub enum ItemKind {
    Invalid = -1,
    None = 0,
    Deagle,
    DualBerettas,
    FiveSeven,
    Glock,
    AK47 = 7,
    AUG,
    Awp,
    Famas,
    G3SG1,
    GalilAr = 13,
    M249,
    M4A4 = 16,
    MAC10,
    P90 = 19,
    MP5 = 23,
    UMP45,
    XM1014,
    PPBizon,
    MAG7,
    Negev,
    SawedOff,
    Tec9,
    ZeusX27,
    P2000,
    MP7,
    MP9,
    Nova,
    P250,
    Shield,
    SCAR20,
    SG556,
    SSG08,
    KnifeGG,
    Knife,
    Flashbang,
    Grenade,
    Smoke,
    Molotov,
    Decoy,
    Incendiary,
    C4,
    Healthshoot = 57,
    KnifeT = 59,
    M4A1S,
    USP,
    CZ75Auto = 63,
    R8Revolver,
    TaGrenade = 68,
    Fists,
    BreachCharge,
    Tablet = 72,
    Melee = 74,
    Axe,
    Hammer,
    Spanner = 78,
    KnifeGhost = 80,
    FireBomb,
    Diversion,
    FragGrenade,
    Snowball,
    BumpMine,
    Bayonet = 500,
    ClassicKnife = 503,
    FlipKnife = 505,
    GutKnife,
    Karambit,
    M9Bayonet,
    TacticalKnife,
    FalchionKnife = 512,
    BowieKnife = 514,
    ButterflyKnife,
    ShadowDaggers,
    CordKnife = 517,
    CanisKnife = 518,
    UrsusKnife = 519,
    NavjaKnife,
    KnifeOutdoor = 521,
    StilettoKnife = 522,
    TalonKnife,
    SkeletonKnife = 525,
    StuddedBloodhound = 5027,
    TSide = 5028,
    CTSide = 5029,
    Sporty = 5030,
    Slick = 5031,
    LeatherWrap = 5032,
    Motocycle = 5033,
    Specialist = 5034,
    Hydra = 5035,
}

impl ItemKind {
    pub const fn is_pistol(&self) -> bool {
        matches!(
            self,
            ItemKind::DualBerettas
                | ItemKind::FiveSeven
                | ItemKind::Glock
                | ItemKind::Tec9
                | ItemKind::P2000
                | ItemKind::P250
                | ItemKind::USP
                | ItemKind::CZ75Auto,
        )
    }

    pub const fn is_heavy_pistol(&self) -> bool {
        matches!(self, ItemKind::Deagle | ItemKind::R8Revolver)
    }

    pub const fn is_rifle(&self) -> bool {
        matches!(
            self,
            ItemKind::AK47
                | ItemKind::AUG
                | ItemKind::Famas
                | ItemKind::G3SG1
                | ItemKind::GalilAr
                | ItemKind::M4A4
                | ItemKind::SCAR20
                | ItemKind::SG556
                | ItemKind::M4A1S,
        )
    }

    pub const fn is_heavy(&self) -> bool {
        matches!(
            self,
            ItemKind::M249
                | ItemKind::XM1014
                | ItemKind::MAG7
                | ItemKind::Negev
                | ItemKind::SawedOff
                | ItemKind::Nova,
        )
    }

    pub const fn is_sub_machine_gun(&self) -> bool {
        matches!(
            self,
            ItemKind::MAC10
                | ItemKind::P90
                | ItemKind::UMP45
                | ItemKind::MP5
                | ItemKind::PPBizon
                | ItemKind::MP7
                | ItemKind::MP9,
        )
    }

    pub const fn label(&self) -> Option<&'static str> {
        let label = match *self {
            ItemKind::Deagle => "Deagle",
            ItemKind::DualBerettas => "Dual Berettas",
            ItemKind::FiveSeven => "Five Seven",
            ItemKind::Glock => "Glock",
            ItemKind::AK47 => "AK47",
            ItemKind::AUG => "AUG",
            ItemKind::Awp => "AWP",
            ItemKind::Famas => "Famas",
            ItemKind::G3SG1 => "G3SG1",
            ItemKind::GalilAr => "Galil AR",
            ItemKind::M249 => "M249",
            ItemKind::M4A4 => "M4A4",
            ItemKind::MAC10 => "MAC-10",
            ItemKind::P90 => "P90",
            ItemKind::UMP45 => "UMP-45",
            ItemKind::MP5 => "MP5",
            ItemKind::XM1014 => "XM1014",
            ItemKind::PPBizon => "PP-Bizon",
            ItemKind::MAG7 => "MAG-7",
            ItemKind::Negev => "Negev",
            ItemKind::SawedOff => "SawedOff",
            ItemKind::Tec9 => "Tec-9",
            ItemKind::ZeusX27 => "Zeus x27",
            ItemKind::P2000 => "P2000",
            ItemKind::MP7 => "MP7",
            ItemKind::MP9 => "MP9",
            ItemKind::Nova => "Nova",
            ItemKind::P250 => "P250",
            ItemKind::SCAR20 => "SCAR-20",
            ItemKind::SG556 => "SG 556",
            ItemKind::SSG08 => "SSG 08",
            ItemKind::Knife => "Knife",
            ItemKind::Flashbang => "Flashbang",
            ItemKind::Grenade => "Grenade",
            ItemKind::Smoke => "Smoke",
            ItemKind::Molotov => "Molotov",
            ItemKind::Decoy => "Decoy",
            ItemKind::Incendiary => "Incendiary",
            ItemKind::C4 => "C4",
            ItemKind::KnifeT => "Knife",
            ItemKind::M4A1S => "M4A1-S",
            ItemKind::USP => "USP",
            ItemKind::CZ75Auto => "CZ75-Auto",
            ItemKind::R8Revolver => "R8 Revolver",
            _ => return None,
        };

        Some(label)
    }
}
