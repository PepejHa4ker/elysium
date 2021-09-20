#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(i32)]
pub enum ItemKind {
    Invalid = -1,
    WeaponNone = 0,
    WeaponDeagle,
    WeaponDualBerettas,
    WeaponFiveSeven,
    WeaponGlock,
    WeaponAK47 = 7,
    WeaponAUG,
    WeaponAwp,
    WeaponFamas,
    WeaponG3SG1,
    WeaponGalilar = 13,
    WeaponM249,
    WeaponM4A4 = 16,
    WeaponMAC10,
    WeaponP90 = 19,
    WeaponMP5 = 23,
    WeaponUMP45,
    WeaponXM1014,
    WeaponPPBizon,
    WeaponMag7,
    WeaponNegev,
    WeaponSawedOff,
    WeaponTec9,
    WeaponZeusX27,
    WeaponP2000,
    WeaponMP7,
    WeaponMP9,
    WeaponNova,
    WeaponP250,
    WeaponShield,
    WeaponSCAR20,
    WeaponSG556,
    WeaponSSG08,
    WeaponKnifeGG,
    WeaponKnife,
    WeaponFlashbang,
    WeaponGrenade,
    WeaponSmoke,
    WeaponMolotov,
    WeaponDecoy,
    WeaponIncendiary,
    WeaponC4,
    WeaponHealthshoot = 57,
    WeaponKnifeT = 59,
    WeaponM4A1S,
    WeaponUSP,
    WeaponCZ75Auto = 63,
    WeaponR8Revolver,
    WeaponTaGrenade = 68,
    WeaponFists,
    WeaponBreachCharge,
    WeaponTablet = 72,
    WeaponMelee = 74,
    WeaponAxe,
    WeaponHammer,
    WeaponSpanner = 78,
    WeaponKnifeGhost = 80,
    WeaponFireBomb,
    WeaponDiversion,
    WeaponFragGrenade,
    WeaponSnowball,
    WeaponBumpMine,
    WeaponBayonet = 500,
    WeaponClassicKnife = 503,
    WeaponFlipKnife = 505,
    WeaponGutKnife,
    WeaponKarambit,
    WeaponM9Bayonet,
    WeaponTacticalKnife,
    WeaponFalchionKnife = 512,
    WeaponBowieKnife = 514,
    WeaponButterflyKnife,
    WeaponShadowDaggers,
    WeaponCordKnife = 517,
    WeaponCanisKnife = 518,
    WeaponUrsusKnife = 519,
    WeaponNavjaKnife,
    WeaponKnifeOutdoor = 521,
    WeaponStilettoKnife = 522,
    WeaponTalonKnife,
    WeaponSkeletonKnife = 525,
    WeaponStuddedBloodhound = 5027,
    WeaponTSide = 5028,
    WeaponCTSide = 5029,
    WeaponSporty = 5030,
    WeaponSlick = 5031,
    WeaponLeatherWrap = 5032,
    WeaponMotocycle = 5033,
    WeaponSpecialist = 5034,
    WeaponHydra = 5035,
}

impl ItemKind {
    pub const fn is_pistol(&self) -> bool {
        use ItemKind::*;

        matches!(
            self,
            WeaponDualBerettas
                | WeaponFiveSeven
                | WeaponGlock
                | WeaponTec9
                | WeaponP2000
                | WeaponP250
                | WeaponUSP
                | WeaponCZ75Auto,
        )
    }

    pub const fn is_heavy_pistol(&self) -> bool {
        use ItemKind::*;

        matches!(self, WeaponDeagle | WeaponR8Revolver)
    }

    pub const fn is_rifle(&self) -> bool {
        use ItemKind::*;

        matches!(
            self,
            WeaponAK47
                | WeaponAUG
                | WeaponFamas
                | WeaponG3SG1
                | WeaponGalilar
                | WeaponM4A4
                | WeaponSCAR20
                | WeaponSG556
                | WeaponM4A1S,
        )
    }

    pub const fn is_heavy(&self) -> bool {
        use ItemKind::*;

        matches!(
            self,
            WeaponM249 | WeaponXM1014 | WeaponMag7 | WeaponNegev | WeaponSawedOff | WeaponNova,
        )
    }

    pub const fn is_sub_machine_gun(&self) -> bool {
        use ItemKind::*;

        matches!(
            self,
            WeaponMAC10
                | WeaponP90
                | WeaponUMP45
                | WeaponMP5
                | WeaponPPBizon
                | WeaponMP7
                | WeaponMP9,
        )
    }

    pub const fn label(&self) -> Option<&'static str> {
        use ItemKind::*;

        let label = match *self {
            WeaponDeagle => "Deagle",
            WeaponDualBerettas => "Dual Berettas",
            WeaponFiveSeven => "Five Seven",
            WeaponGlock => "Glock",
            WeaponAK47 => "AK47",
            WeaponAUG => "AUG",
            WeaponAWP => "AWP",
            WeaponFamas => "Famas",
            WeaponG3SG1 => "G3SG1",
            WeaponGalilAR => "Galil AR",
            WeaponM249 => "M249",
            WeaponM4A4 => "M4A4",
            WeaponMAC10 => "MAC-10",
            WeaponP90 => "P90",
            WeaponUMP45 => "UMP-45",
            WeaponMP5 => "MP5",
            WeaponXM1014 => "XM1014",
            WeaponPPBizon => "PP-Bizon",
            WeaponMAG7 => "MAG-7",
            WeaponNegev => "Negev",
            WeaponSawedOff => "SawedOff",
            WeaponTec9 => "Tec-9",
            WeaponZeusX27 => "Zeus x27",
            WeaponP2000 => "P2000",
            WeaponMP4 => "MP7",
            WeaponMP9 => "MP9",
            WeaponNova => "Nova",
            WeaponP250 => "P250",
            WeaponSCAR20 => "SCAR-20",
            WeaponSG556 => "SG 556",
            WeaponSSG08 => "SSG 08",
            WeaponKnife => "Knife",
            WeaponFlashhbang => "Flashbang",
            WeaponGrenade => "Grenade",
            WeaponSmoke => "Smoke",
            WeaponMolotov => "Molotov",
            WeaponDecoy => "Decoy",
            WeaponIncendiary => "Incendiary",
            WeaponC4 => "C4",
            WeaponKnifeT => "Knife",
            WeaponM4A1S => "M4A1-S",
            WeaponUSP => "USP",
            WeaponCZ75Auto => "CZ75-Auto",
            WeaponR8Revoler => "R8 Revolver",
            WeaponBayonet => "Bayonet",
            WeaponFlipKnife => "Flip Knife",
            WeaponGutKnife => "Gut Knife",
            WeaponKarambit => "Karambit",
            WeaponM9Bayonet => "M9 Bayonet",
            WeaponTacticalKnife => "Tactical Knife",
            WeaponFalchionKnife => "Falchion Knife",
            WeaponBowie => "Bowie Knife",
            WeaponButterflyKnife => "Butterfly Knife",
            WeaponShadowDaggers => "Shadow Daggers",
            WeaponUrsusKnife => "Ursus Knife",
            WeaponNavajaKnife => "NavajaKnife",
            WeaponStilettoKnife => "Stiletto Knife",
            WeaponTalonKnife => "Talon Knife",
            WeaponClassicKnife => "Classic Knife",
            WeaponGhostKnife => "Ghost Knife",
            BloodhoundGloves => "BloodhoundGloves",
            _ => return None,
        };

        Some(label)
    }
}
