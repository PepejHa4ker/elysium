pub mod library {
    pub const CLIENT: &str = "./csgo/bin/linux64/client_client.so\0";
    pub const ENGINE: &str = "./bin/linux64/engine_client.so\0";
    pub const FS_STDIO: &str = "./bin/linux64/filesystem_stdio_client.so\0";
    pub const INPUTSYSTEM: &str = "./bin/linux64/inputsystem_client.so\0";
    pub const LOCALIZE: &str = "./bin/linux64/localize_client.so\0";
    pub const MATCHMAKING: &str = "./csgo/bin/linux64/matchmaking_client.so\0";
    pub const MATERIALSYSTEM: &str = "./bin/linux64/materialsystem_client.so\0";
    pub const PANORAMA: &str = "./bin/linux64/panorama_client.so\0";
    pub const SERVERBROWSER: &str = "./bin/linux64/serverbrowser_client.so\0";
    pub const VGUIMATSURFACE: &str = "./bin/linux64/vguimatsurface_client.so\0";
    pub const VGUI2: &str = "./bin/linux64/vgui2_client.so\0";
    pub const VPHYSICS: &str = "./bin/linux64/vphysics_client.so\0";
    pub const SDL: &str = "libSDL2-2.0.so.0\0";

    pub mod sdl {
        pub const SWAPWINDOW: &[u8] = b"SDL_GL_SwapWindow\0";
        pub const POLLEVENT: &[u8] = b"SDL_PollEvent\0";
    }

    pub const INTERFACES: &str = "s_pInterfaceRegs\0";
}

pub mod interface {
    pub const VCLIENT: &str = "VClient";
    pub const VENGINECVAR: &str = "VEngineCvar";
    pub const VENGINECLIENT: &str = "VEngineClient";
    pub const VGUIPANEL: &str = "VGUI_Panel";
    pub const VCLIENTENTITYLIST: &str = "VClientEntityList";
    pub const VENGINEVGUI: &str = "VEngineVGui";
    pub const VENGINEMODEL: &str = "VEngineModel";
    pub const VMODELINFOCLIENT: &str = "VModelInfoClient";
    pub const VMATERIALSYSTEM: &str = "VMaterialSystem";
    pub const IENGINESOUNDCLIENT: &str = "IEngineSoundClient";
    pub const ENGINETRACECLIENT: &str = "EngineTraceClient";
    pub const GAMEMOVEMENT: &str = "GameMovement";
    pub const VPHYSICSSURFACEPROPS: &str = "VPhysicsSurfaceProps";

    pub const VCLIENTPREDICTION: &str = "VClientPrediction001";
    pub const GAMEVENTSMANAGER: &str = "GAMEEVENTSMANAGER002";
}

pub mod offset {
    pub const CREATE_MOVE: isize = 25;
    pub const FRAME_STAGE_NOTIFY: isize = 37;
}

pub mod var {
    use std::ffi::CStr;

    macro_rules! var {
        ($var:expr) => {
            unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($var, "\0").as_bytes())
            }
        };
    }

    pub type Var = &'static CStr;

    pub const CHEATS: Var = var!("sv_cheats");
    pub const FFA: Var = var!("mp_teammates_are_enemies");
    pub const GRAVITY: Var = var!("sv_gravity");
    pub const INFINITE_AMMO: Var = var!("sv_infinite_ammo");
    pub const LOST_FOCUS_SLEEP: Var = var!("engine_no_focus_sleep");
    pub const MODEL_STATS_OVERLAY: Var = var!("r_drawmodelstatsoverlay");
    pub const PANORAMA_BLUR: Var = var!("@panorama_disable_blur");
    pub const PHYSICS_TIMESCALE: Var = var!("cl_phys_timescale");
    // TODO: Don't touch the variable.
    pub const POST_PROCESS: Var = var!("mat_postprocess_enable");
    pub const RAGDOLL_GRAVITY: Var = var!("cl_ragdoll_gravity");
    pub const SHOW_IMPACTS: Var = var!("sv_showimpacts");
}

pub mod skybox {
    pub const SKYBOXES: [&str; 27] = [
        "None",
        // sky_l4d_rural02_ldr
        "Black",
        "cs_baggage_skybox_",
        "cs_tibet",
        "embassy",
        "italy",
        "jungle",
        "nukeblank",
        "office",
        "sky_cs15_daylight01_hdr",
        "sky_cs15_daylight02_hdr",
        "sky_cs15_daylight03_hdr",
        "sky_cs15_daylight04_hdr",
        "sky_csgo_cloudy01",
        "sky_csgo_night_flat",
        "sky_csgo_night02",
        "sky_csgo_night02b",
        "sky_day02_05",
        "sky_day02_05_hdr",
        "sky_dust",
        "sky_hr_aztec",
        "sky_lunacy",
        "sky_venice",
        "vertigo",
        "vertigo_hdr",
        "vertigoblue_hdr",
        "vietnam",
    ];
}
