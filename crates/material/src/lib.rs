macro_rules! material_kinds {
    ($($name:ident => $string:literal),*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum MaterialKind {
            $(
                $name,
            )*
        }

        impl MaterialKind {
            pub fn as_ptr(&self) -> *const u8 {
                match self {
                    $(
                        MaterialKind::$name => concat!($string, "\0").as_ptr(),
                    )*
                }
            }
        }
    };
}

material_kinds! {
    Darude => "models/inventory_items/music_kit/darude_01/mp3_detail",
    Energy => "effects/energyball",
    Flat => "debug/debugdrawflat",
    Plastic => "models/inventory_items/trophy_majors/gloss",
    Pulse => "dev/screenhighlight_pulse",
    Shaded => "debug/debugambientcube"
}

/*materials! {
    Normal => "VertexLitGeneric",
    Flat => "UnlitGeneric",
    Chrome => "VertixLitGeneric" {
        "$envmap": "env_cubemap",
    },
    Glow => "VertexLitGeneric" {
        "$additive": "1",
        "$envmap": "models/effects/cube_white",
        "$envmapfresnel": "1",
        "$alpha": .8,
    },
}

// this wont work for materials with proxies!
struct KeyValues {
    additive: Option<bool>,
    alpha: Option<f32>,
    ambientonly: Option<bool>,
    basetexture: Option<&'static str>,
    basemapalphaphongmask: Option<bool>,
    envmap: Option<&'static str>,
    envmapfresnel: Option<bool>,
    flat: Option<bool>,
    halfambert: Option<bool>,
    ignorez: Option<bool>,
    model: Option<bool>,
    nocull: Option<bool>,
    nofog: Option<bool>,
    normalmapalphaenvmapmask: Option<bool>,
    phong: Option<bool>,
    pearlescent: Option<i32>,
    selfillum: Option<bool>,
    znearer: Option<bool>,
}*/
