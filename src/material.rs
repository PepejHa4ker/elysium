use crate::managed::{handle, Managed};
use core::mem::MaybeUninit;

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

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub unsafe fn var_unchecked(
        &self,
        name: *const u8,
        found: &mut bool,
        complain: bool,
    ) -> Option<MaterialVar> {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Material,
            name: *const u8,
            found: *mut bool,
            complain: bool,
        ) -> Option<MaterialVar>;

        self.virtual_entry::<Fn>(11)(self.as_ptr(), name, found, complain)
    }

    pub fn var(&self, name: *const u8) -> Option<MaterialVar> {
        unsafe {
            let mut exists = false;
            let var = self.var_unchecked(name, &mut exists, true);

            exists.then(|| var).flatten()
        }
    }

    fn flag(&self, flag: i32, enabled: bool) {
        type Fn = unsafe extern "C" fn(this: *const handle::Material, flag: i32, enabled: bool);

        unsafe {
            self.virtual_entry::<Fn>(29)(self.as_ptr(), flag, enabled);
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
        type AlphaFn = unsafe extern "C" fn(this: *const handle::Material, alpha: f32);
        type RgbFn = unsafe extern "C" fn(this: *const handle::Material, r: f32, g: f32, b: f32);

        unsafe {
            let [r, g, b, a] = rgba;

            self.virtual_entry::<RgbFn>(28)(self.as_ptr(), r, g, b);
            self.virtual_entry::<AlphaFn>(27)(self.as_ptr(), a);

            if let Some(var) = self.var(ENV_TINT_MAP.as_ptr()) {
                var.set_tint(r, g, b);
            }
        }
    }

    pub fn get_color(&self) -> [f32; 4] {
        type AlphaFn = unsafe extern "C" fn(this: *const handle::Material) -> f32;
        type RgbFn = unsafe extern "C" fn(
            this: *const handle::Material,
            r: *mut f32,
            g: *mut f32,
            b: *mut f32,
        );

        unsafe {
            let mut color: [MaybeUninit<f32>; 4] = MaybeUninit::uninit_array();

            self.virtual_entry::<RgbFn>(45)(
                self.as_ptr(),
                color[0].as_mut_ptr(),
                color[1].as_mut_ptr(),
                color[2].as_mut_ptr(),
            );

            color[3].write(self.virtual_entry::<AlphaFn>(44)(self.as_ptr()));

            MaybeUninit::array_assume_init(color)
        }
    }
}

pub const ENV_TINT_MAP: &str = "$envmaptint\0";

// https://developer.valvesoftware.com/wiki/Material_Flags
pub const MATERIAL_VAR_NO_DRAW: i32 = 0x0004;
pub const MATERIAL_VAR_IGNOREZ: i32 = 0x8000;
pub const MATERIAL_VAR_WIREFRAME: i32 = 0x10000000;
