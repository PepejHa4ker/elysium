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
