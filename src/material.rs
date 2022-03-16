use crate::managed::{handle, Managed};
use core::mem::MaybeUninit;
use providence_util::virtual_table;

pub use materials::Materials;
pub use var::MaterialVar;

mod materials;
mod var;

/// A material.
#[derive(Debug)]
#[repr(transparent)]
pub struct Material(Managed<handle::Material>);

impl Material {
    pub fn new(ptr: *mut handle::Material) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Material) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Material {
        self.0.as_ptr()
    }

    virtual_table! {
        fn var_unchecked[11](name: *const u8, found: *mut bool, complain: bool) -> Option<MaterialVar>;
        fn alpha_unchecked[27](a: f32) -> ();
        fn rgb_unchecked[28](r: f32, g: f32, b: f32) -> ();
        fn flag_unchecked[29](flag: i32, enabled: bool) -> ();
        fn get_alpha_unchecked[44]() -> f32;
        fn get_rgb_unchecked[45](r: *mut f32, g: *mut f32, b: *mut f32) -> ();
    }

    pub fn var(&self, name: *const u8) -> Option<MaterialVar> {
        unsafe {
            let mut exists = false;
            let var = self.var_unchecked(name, &mut exists, true);

            exists.then(|| var).flatten()
        }
    }

    fn flag(&self, flag: i32, enabled: bool) {
        unsafe {
            self.flag_unchecked(flag, enabled);
        }
    }

    pub fn no_draw(&self, enabled: bool) {
        self.flag(MATERIAL_VAR_NO_DRAW, enabled);
    }

    pub fn ignore_z(&self, enabled: bool) {
        self.flag(MATERIAL_VAR_IGNOREZ, enabled);
    }

    pub fn wireframe(&self, enabled: bool) {
        self.flag(MATERIAL_VAR_WIREFRAME, enabled);
    }

    pub fn color(&self, rgba: [f32; 4]) {
        let [r, g, b, a] = rgba;

        unsafe {
            self.alpha_unchecked(a);
            self.rgb_unchecked(r, g, b);
        }

        if let Some(var) = self.var(ENV_TINT_MAP.as_ptr()) {
            var.set_tint(r, g, b);
        }
    }

    pub fn get_color(&self) -> [f32; 4] {
        unsafe {
            let mut color: [MaybeUninit<f32>; 4] = MaybeUninit::uninit_array();

            self.get_rgb_unchecked(
                color[0].as_mut_ptr(),
                color[1].as_mut_ptr(),
                color[2].as_mut_ptr(),
            );

            color[3].write(self.get_alpha_unchecked());

            MaybeUninit::array_assume_init(color)
        }
    }
}

pub const ENV_TINT_MAP: &str = "$envmaptint\0";

// https://developer.valvesoftware.com/wiki/Material_Flags
pub const MATERIAL_VAR_NO_DRAW: i32 = 0x0004;
pub const MATERIAL_VAR_IGNOREZ: i32 = 0x8000;
pub const MATERIAL_VAR_WIREFRAME: i32 = 0x10000000;
