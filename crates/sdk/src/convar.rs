use crate::{Pad, UtlVec};
use core::marker::PhantomData;
use frosting::ffi::vtable;

mod sealed {
    use super::Var;

    pub trait Sealed: Sized {
        fn read(var: &Var<Self>) -> Self;
        fn write(self, var: &Var<Self>);
    }

    impl Sealed for f32 {
        #[inline]
        fn read(var: &Var<f32>) -> Self {
            var.read_f32()
        }

        #[inline]
        fn write(self, var: &Var<f32>) {
            var.write_f32(self)
        }
    }

    impl Sealed for i32 {
        #[inline]
        fn read(var: &Var<i32>) -> Self {
            var.read_i32()
        }

        #[inline]
        fn write(self, var: &Var<i32>) {
            var.write_i32(self)
        }
    }

    impl Sealed for bool {
        #[inline]
        fn read(var: &Var<bool>) -> Self {
            var.read_i32() != 0
        }

        #[inline]
        fn write(self, var: &Var<bool>) {
            var.write_i32(self as i32)
        }
    }
}

/// valid types config variables can store
pub trait Kind: sealed::Sealed {}

impl Kind for f32 {}
impl Kind for i32 {}
impl Kind for bool {}

#[derive(Debug)]
#[repr(C)]
struct VTable<T> {
    _pad0: vtable::Pad<15>,
    read_f32: unsafe extern "C" fn(this: *const Var<T>) -> f32,
    write_f32: unsafe extern "C" fn(this: *const Var<T>, value: f32),
    _pad1: vtable::Pad<1>,
    read_i32: unsafe extern "C" fn(this: *const Var<T>) -> i32,
    write_i32: unsafe extern "C" fn(this: *const Var<T>, value: i32),
}

/// config variable
#[derive(Debug)]
#[repr(C)]
pub struct Var<T> {
    /// blah blah static
    vtable: *const VTable<T>,
    _pad0: Pad<40>,
    pub change_callback: Option<unsafe extern "C" fn()>,
    pub parent: *const Var<()>,
    pub default_value: *const u8,
    pub string: *const u8,
    _pad1: Pad<28>,
    pub on_change_callbacks: UtlVec<unsafe extern "C" fn()>,
    // we do be owning T, tho
    _phantom: PhantomData<T>,
}

impl<T> Var<T> {
    #[inline]
    fn read_f32(&self) -> f32 {
        unsafe { ((*self.vtable).read_f32)(self) }
    }

    #[inline]
    fn write_f32(&self, value: f32) {
        unsafe { ((*self.vtable).write_f32)(self, value) }
    }

    #[inline]
    fn read_i32(&self) -> i32 {
        unsafe { ((*self.vtable).read_i32)(self) }
    }

    #[inline]
    fn write_i32(&self, value: i32) {
        unsafe { ((*self.vtable).write_i32)(self, value) }
    }
}

impl<T> Var<T>
where
    T: Kind,
{
    /// read the config variable
    #[inline]
    pub fn read(&self) -> T {
        <T as sealed::Sealed>::read(self)
    }

    /// write `value` to the config variable
    #[inline]
    pub fn write(&self, value: T) {
        <T as sealed::Sealed>::write(value, self)
    }
}

macro_rules! vars {
    ($($name:ident: $type:ty => $string:literal),*) => {
        /// config variable name
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum VarKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                // doc's alias is the same as it's name, cring
                //#[doc(alias = $string)]
                $name,
            )*
        }

        const VARS: phf::Map<&'static str, VarKind> = phf::phf_map! {
            $(
                $string => VarKind::$name,
            )*
        };

        impl VarKind {
            /// map a string into an var we're interested in
            #[inline]
            pub fn from_str(var: &str) -> Option<Self> {
                VARS.get(var).cloned()
            }

            /// returns the actual game variable this maps to
            #[inline]
            pub const fn as_nul_str(&self) -> &'static str {
                match self {
                    $(
                        VarKind::$name => concat!($string, "\0"),
                    )*
                }
            }

            /// returns the actual game variable this maps to
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                let string = self.as_nul_str();

                unsafe { string.get_unchecked(0..string.len().saturating_sub(1)) }
            }

            /// returne a pointer to the var
            #[inline]
            pub const fn as_ptr(&self) -> *const u8 {
                self.as_nul_str().as_ptr()
            }
        }

        /// config variables
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct Vars {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                // doc's alias is the same as it's name, cring
                //#[doc(alias = $string)]
                pub $name: &'static Var<$type>,
            )*
        }

        impl Vars {
            /// load all config variables
            #[inline]
            pub fn from_loader<L>(mut loader: L) -> Self
            where
                L: FnMut(VarKind) -> *const ()
            {
                $(
                    let $name = {
                        let var = loader(VarKind::$name).cast::<Var<$type>>().as_mut();

                        if var.is_null() {
                            panic!(concat!("config variable `", stringify!($name), "` is null"));
                        }

                        unsafe { &mut *var }
                    };
                )*

                Self { $($name,)* }
            }
        }
    };
}

vars! {
    alien_blood: bool => "violence_ablood",
    allow_developer: bool => "sv_max_allowed_developer",
    auto_help: bool => "cl_autohelp",
    cheats: bool => "sv_cheats",
    csm: bool => "cl_csm_enabled",
    csm_shadows: bool => "cl_csm_shadows",
    decals: bool => "r_drawdecals",
    do_interp: bool => "cl_interpolate",
    engine_sleep: bool => "engine_no_focus_sleep",
    ffa: bool => "mp_teammates_are_enemies",
    feet_shadows: bool => "cl_foot_contact_shadows",
    freeze_cam: bool => "cl_disablefreezecam",
    gravity: f32 => "sv_gravity",
    horizontal_speed: f32 => "cl_sidespeed",
    html_motd: bool => "cl_disablehtmlmotd",
    hud: bool => "cl_drawhud",
    human_blood: bool => "violence_hblood",
    interp: f32 => "cl_interp",
    interp_ratio: f32 => "cl_interp_ratio",
    jiggle_bones: bool => "r_jiggle_bones",
    lag_comp: f32 => "cl_lagcompensation",
    developer: bool => "developer",
    max_interp_ratio: f32 => "sv_client_max_interp_ratio",
    max_lag_comp: f32 => "sv_maxunlag",
    min_interp_ratio: f32 => "sv_client_min_interp_ratio",
    model_stats: i32 => "r_drawmodelstatsoverlay",
    panorama_blur: bool => "@panorama_disable_blur",
    physics_timescale: f32 => "cl_phys_timescale",
    prop_shadows: bool => "cl_csm_static_prop_shadows",
    rain: bool => "r_drawrain",
    recoil_scale: f32 => "weapon_recoil_scale",
    ragdoll_gravity: f32 => "cl_ragdoll_gravity",
    ropes: bool => "r_drawropes",
    rope_shadows: bool => "cl_csm_rope_shadows",
    shadows: bool => "r_shadows",
    show_grenade_path: bool => "cl_grenadepreview",
    show_help: bool => "cl_showhelp",
    show_impacts: bool => "sv_showimpacts",
    sprites: bool => "r_drawsprites",
    skybox3d: bool => "r_3dsky",
    sprite_shadows: bool => "cl_csm_sprite_shadows",
    translucent_renderables: bool => "r_drawtranslucentrenderables",
    translucent_world: bool => "r_drawtranslucentworld",
    update_rate: f32 => "cl_updaterate",
    underwater_overlay: bool => "r_drawunderwateroverlay",
    vertical_speed: f32 => "cl_forwardspeed",
    viewmodel_shadows: bool => "cl_csm_viewmodel_shadows",
    water_fog: bool => "fog_enable_water_fog",
    world_shadows: bool => "cl_csm_world_shadows"
}
