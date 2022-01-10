use crate::managed::{handle, Managed};

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

    pub fn var(&self, name: *const i8) -> Option<MaterialVar> {
        let mut found = false;
        let ptr = unsafe { self.var_unchecked(name, &mut found, true) };

        if found {
            MaterialVar::new(ptr)
        } else {
            None
        }
    }

    pub unsafe fn var_unchecked(
        &self,
        name: *const i8,
        found: &mut bool,
        complain: bool,
    ) -> *mut handle::MaterialVar {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Material,
            name: *const i8,
            found: *mut bool,
            complain: bool,
        ) -> *mut handle::MaterialVar;

        self.virtual_entry::<Fn>(11)(self.as_ptr(), name, found, complain)
    }

    pub fn set_alpha(&self, a: f32) {
        type Fn = unsafe extern "C" fn(this: *const handle::Material, a: f32);

        unsafe {
            self.virtual_entry::<Fn>(27)(self.as_ptr(), a);
        }
    }

    pub fn set_rgb(&self, r: f32, g: f32, b: f32) {
        type Fn = unsafe extern "C" fn(this: *const handle::Material, r: f32, g: f32, b: f32);

        unsafe {
            self.virtual_entry::<Fn>(28)(self.as_ptr(), r, g, b);
        }
    }

    pub fn set_rgba(&self, r: f32, g: f32, b: f32, a: f32) {
        self.set_rgb(r, g, b);
        self.set_alpha(a);
    }

    pub fn set_rgba8(&self, r: u8, g: u8, b: u8, a: u8) {
        self.set_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        self.set_alpha(a as f32 / 255.0);
    }

    pub fn set_tint(&self, r: f32, g: f32, b: f32) {
        if let Some(var) = self.var("$envmaptint\0".as_ptr() as _) {
            var.set_tint(r, g, b);
        }
    }

    pub fn set_flag(&self, flag: i32, enabled: bool) {
        type Fn = unsafe extern "C" fn(this: *const handle::Material, flag: i32, enabled: bool);

        unsafe {
            self.virtual_entry::<Fn>(29)(self.as_ptr(), flag, enabled);
        }
    }

    pub fn set_ignore_z(&self, enabled: bool) {
        self.set_flag(1 << 15, enabled);
    }

    pub fn set_wireframe(&self, enabled: bool) {
        self.set_flag(1 << 28, enabled);
    }

    pub fn get_alpha(&self) -> f32 {
        type Fn = unsafe extern "C" fn(this: *const handle::Material, alpha: *mut f32);

        let mut alpha = 0.0;

        unsafe {
            self.virtual_entry::<Fn>(44)(self.as_ptr(), &mut alpha);
        }

        alpha
    }

    pub fn get_rgb(&self) -> [f32; 3] {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Material,
            r: *mut f32,
            g: *mut f32,
            b: *mut f32,
        );

        let mut rgb = [0.0; 3];

        unsafe {
            self.virtual_entry::<Fn>(45)(
                self.as_ptr(),
                rgb.get_unchecked_mut(0),
                rgb.get_unchecked_mut(1),
                rgb.get_unchecked_mut(2),
            );
        }

        rgb
    }

    pub fn get_rgba(&self) -> [f32; 4] {
        let [r, g, b] = self.get_rgb();
        let a = self.get_alpha();

        [r, g, b, a]
    }
}
