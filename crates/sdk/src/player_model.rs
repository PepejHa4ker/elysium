// https://www.unknowncheats.me/forum/counterstrike-global-offensive/470973-player-agents.html
// https://github.com/Franc1sco/Franug-AgentsChooser/blob/master/csgo_agentschooser.sp

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Series {
    BrazilianFirstBattalion,
    EliteCrew,
    FBI,
    FBISniper,
    FBISWAT,
    GendarmerieNationale,
    GuerrillaWarfare,
    KSK,
    NSWCSEAL,
    NZSAS,
    Phoenix,
    SAS,
    SEALFrogman,
    SWAT,
    Sabre,
    SabreFootsoldier,
    TACPCavalry,
    TheProfessionals,
    USAFTACP,
}

impl Series {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Series::BrazilianFirstBattalion => "Brazilian 1st Battalion",
            Series::EliteCrew => "Elite Crew",
            Series::FBI => "FBI",
            Series::FBISniper => "FBI Sniper",
            Series::FBISWAT => "FBI SWAT",
            Series::GendarmerieNationale => "Gendarmerie Nationale",
            Series::GuerrillaWarfare => "Guerrilla Warfare",
            Series::KSK => "KSK",
            Series::NSWCSEAL => "NSWC SEAL",
            Series::NZSAS => "NZSAS",
            Series::Phoenix => "Phoenix",
            Series::SAS => "SAS",
            Series::SEALFrogman => "SEAL Frogman",
            Series::SWAT => "SWAT",
            Series::Sabre => "Sabre",
            Series::SabreFootsoldier => "Sabre Footsoldier",
            Series::TACPCavalry => "TACP Cavalry",
            Series::TheProfessionals => "The Professionals",
            Series::USAFTACP => "USAF TACP",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Category {
    Distinguished,
    Exceptional,
    Master,
    Superior,
}

impl Category {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Category::Distinguished => "distinguished",
            Category::Exceptional => "exceptional",
            Category::Master => "master",
            Category::Superior => "superior",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Team {
    CounterTerrorist,
    Terrorist,
}

impl Team {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Team::CounterTerrorist => "counter terrorist",
            Team::Terrorist => "terrorist",
        }
    }
}

macro_rules! models {
    ($($variant:ident => {
        category: Category::$category:ident,
        label: $label:literal,
        mdl: $mdl:literal,
        series: Series::$series:ident,
        team: Team::$team:ident,
    }),*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum PlayerModel {
            $($variant,)*
        }

        impl PlayerModel {
            #[inline]
            pub const fn category(&self) -> Category {
                match self {
                    $(PlayerModel::$variant => Category::$category,)*
                }
            }

            #[inline]
            pub const fn label(&self) -> &'static str {
                match self {
                    $(PlayerModel::$variant => $label,)*
                }
            }

            #[inline]
            pub const fn mdl(&self) -> &'static str {
                match self {
                    $(PlayerModel::$variant => $mdl,)*
                }
            }

            #[inline]
            pub const fn series(&self) -> Series {
                match self {
                    $(PlayerModel::$variant => Series::$series,)*
                }
            }

            #[inline]
            pub const fn team(&self) -> Team {
                match self {
                    $(PlayerModel::$variant => Team::$team,)*
                }
            }
        }
    };
}

models! {
    // counter terrorist master series
    ChefdEscadronRouchard => {
        category: Category::Master,
        label: "Chef d'Escadron Rouchard",
        mdl: "models/player/custom_player/legacy/ctm_gendarmerie_variantc.mdl\0",
        series: Series::GendarmerieNationale,
        team: Team::CounterTerrorist,
    },
    CmdrFrankWetSoxBaroud => {
        category: Category::Master,
        label: "Cmdr. Frank 'Wet Sox' Baroud",
        mdl: "models/player/custom_player/legacy/ctm_diver_variantb.mdl\0",
        series: Series::SEALFrogman,
        team: Team::CounterTerrorist,
    },
    CmdrDavidaGogglesFernandez => {
        category: Category::Master,
        label: "Cmdr. Davida 'Goggles' Fernandez",
        mdl: "models/player/custom_player/legacy/ctm_diver_varianta.mdl\0",
        series: Series::SEALFrogman,
        team: Team::CounterTerrorist,
    },
    LtCommanderRicksaw => {
        category: Category::Master,
        label: "Lt. Commander Ricksaw",
        mdl: "models/player/custom_player/legacy/ctm_st6_varianti.mdl\0",
        series: Series::NSWCSEAL,
        team: Team::CounterTerrorist,
    },
    SpecialAgentAva => {
        category: Category::Master,
        label: "Special Agent Ava",
        mdl: "models/player/custom_player/legacy/ctm_fbi_variantb.mdl\0",
        series: Series::FBI,
        team: Team::CounterTerrorist,
    },
    CmdrMaeDeadColdJamison => {
        category: Category::Master,
        label: "Cmdr. Mae 'Dead Cold' Jamison",
        mdl: "models/player/custom_player/legacy/ctm_swat_variante.mdl\0",
        series: Series::SWAT,
        team: Team::CounterTerrorist,
    },
    // terrorist master series
    VypaSistaOfTheRevolution => {
        category: Category::Master,
        label: "Vypa Sista of the Revolution",
        mdl: "models/player/custom_player/legacy/tm_jungle_raider_variante.mdl\0",
        series: Series::GuerrillaWarfare,
        team: Team::Terrorist,
    },
    MediumRareCrasswater => {
        category: Category::Master,
        label: "'Medium Rare' Crasswater",
        mdl: "models/player/custom_player/legacy/tm_jungle_raider_variantb2.mdl\0",
        series: Series::GuerrillaWarfare,
        team: Team::Terrorist,
    },
    CrasswaterTheForgotten => {
        category: Category::Master,
        label: "Crasswater The Forgotten",
        mdl: "models/player/custom_player/legacy/tm_jungle_raider_variantb.mdl\0",
        series: Series::GuerrillaWarfare,
        team: Team::Terrorist,
    },
    TheDoctorRomanov => {
        category: Category::Master,
        label: "'The Doctor' Romanov",
        mdl: "models/player/custom_player/legacy/tm_balkan_varianth.mdl\0",
        series: Series::Sabre,
        team: Team::Terrorist,
    },
    TheEliteMrMuhlik => {
        category: Category::Master,
        label: "The Elite Mr. Muhlik",
        mdl: "models/player/custom_player/legacy/tm_leet_variantf.mdl\0",
        series: Series::EliteCrew,
        team: Team::Terrorist,
    },
    SirBloodyMiamiDarryl => {
        category: Category::Master,
        label: "Sir Bloody Miami Darryl",
        mdl: "models/player/custom_player/legacy/tm_professional_varf.mdl\0",
        series: Series::TheProfessionals,
        team: Team::Terrorist,
    },
    SirBloodySkullheadDarryl => {
        category: Category::Master,
        label: "Sir Bloody Skullhead Darryl",
        mdl: "models/player/custom_player/legacy/tm_professional_varf2.mdl\0",
        series: Series::TheProfessionals,
        team: Team::Terrorist,
    },
    SirBloodyDarrylRoyale => {
        category: Category::Master,
        label: "Sir Bloody Darryl Royale",
        mdl: "models/player/custom_player/legacy/tm_professional_varf3.mdl\0",
        series: Series::TheProfessionals,
        team: Team::Terrorist,
    },
    SirBloodyLoudmouthDarryl => {
        category: Category::Master,
        label: "Sir Bloody Loudmouth Darryl",
        mdl: "models/player/custom_player/legacy/tm_professional_varf4.mdl\0",
        series: Series::TheProfessionals,
        team: Team::Terrorist,
    }
}
