macro_rules! libraries {
    ($($name:ident => $string:literal),*) => {
        /// library
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum LibraryKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $name,
            )*
        }

        impl LibraryKind {
            #[inline]
            pub const fn as_nul_str(&self) -> &'static str {
                match self {
                    $(
                        LibraryKind::$name => concat!($string, "\0"),
                    )*
                }
            }

            /// returns the library's path
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                let string = self.as_nul_str();

                unsafe { string.get_unchecked(0..string.len().saturating_sub(1)) }
            }

            /// returns a pointer to this library's path
            #[inline]
            pub const fn as_ptr(&self) -> *const u8 {
                self.as_nul_str().as_ptr()
            }
        }
    }
}

libraries! {
    Client => "./csgo/bin/linux64/client_client.so",
    Engine => "./bin/linux64/engine_client.so",
    Filesystem => "./bin/linux64/filesystem_stdio_client.so",
    Input => "./bin/linux64/inputsystem_client.so",
    Localize => "./bin/linux64/localize_client.so",
    Matchmaking => "./csgo/bin/linux64/matchmaking_client.so",
    Material => "./bin/linux64/materialsystem_client.so",
    Panorama => "./bin/linux64/panorama_gl_client.so",
    Physics => "./bin/linux64/vphysics_client.so",
    Server => "./bin/linux64/serverbrowser_client.so",
    Surface => "./bin/linux64/vguimatsurface_client.so",
    Tier0 => "./bin/linux64/libtier0_client.so",
    VGui => "./bin/linux64/vgui2_client.so"
}

macro_rules! interfaces {
    ($(($ident:ident, $field:ident) => ($library:ident, $string:literal)),*) => {
        /// interface
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum InterfaceKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $ident,
            )*
        }

        impl InterfaceKind {
            #[inline]
            pub const fn as_nul_str(&self) -> &'static str {
                match self {
                    $(
                        InterfaceKind::$ident => concat!($string, "\0"),
                    )*
                }
            }

            /// returns the interfaces library
            #[inline]
            pub const fn library(&self) -> LibraryKind {
                match self {
                    $(
                        InterfaceKind::$ident => LibraryKind::$library,
                    )*
                }
            }

            /// returns the interfaces name
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                let string = self.as_nul_str();

                unsafe { string.get_unchecked(0..string.len().saturating_sub(1)) }
            }

            /// returns a pointer to this interfaces name
            #[inline]
            pub const fn as_ptr(&self) -> *const u8 {
                self.as_nul_str().as_ptr()
            }
        }

        #[derive(Debug)]
        #[non_exhaustive]
        pub struct Interfaces {
            $(
                pub $field: *const u8,
            )*
        }

        impl Interfaces {
            #[inline]
            pub unsafe fn from_loader<L>(mut loader: L) -> Self
            where
                L: FnMut(InterfaceKind) -> *const ()
            {
                Self { $(
                    $field: loader(InterfaceKind::$ident).cast(),
                )* }
            }
        }
    }
}

interfaces! {
    (Client, client) => (Client, "VClient"),
    (ConVar, convar) => (Material, "VEngineCvar"),
    (Debug, debug) => (Engine, "VDebugOverlay"),
    (Effects, effects) => (Engine, "VEngineEffects"),
    (Engine, engine) => (Engine, "VEngineClient"),
    (EntityList, entity_list) => (Client, "VClientEntityList"),
    (Events, events) => (Engine, "GAMEEVENTSMANAGER002"),
    (Filesystem, filesystem) => (Filesystem, "VFileSystem"),
    (InputInternal, input_internal) => (VGui, "VGUI_InputInternal"),
    (InputSystem, input_system) => (Input, "InputSystemVersion"),
    (Kinds, kinds) => (Matchmaking, "VENGINE_GAMETYPES_VERSION002"),
    (Localize, localize) => (Localize, "Localize_"),
    (Material, material) => (Material, "VMaterialSystem"),
    (ModelInfo, model_info) => (Engine, "VModelInfoClient"),
    (ModelRender, model_render) => (Engine, "VEngineModel"),
    (Movement, movement) => (Client, "GameMovement"),
    (Panel, panel) => (VGui, "VGUI_Panel"),
    (Panorama, panorama) => (Panorama, "PanoramaUIEngine001"),
    (Physics, physics) => (Physics, "VPhysicsSurfaceProps"),
    (Prediction, prediction) => (Client, "VClientPrediction001"),
    (Sound, sound) => (Engine, "IEngineSoundClient"),
    (Surface, surface) => (Surface, "VGUI_Surface"),
    (Trace, trace) => (Engine, "EngineTraceClient"),
    (VGui, vgui) => (Engine, "VEngineVGui")
}
