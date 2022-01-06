use core::ptr::NonNull;

extern "C" {
    /// Raw handle to a material.
    pub type RawMaterialVar;
}

unsafe impl Send for RawMaterialVar {}
unsafe impl Sync for RawMaterialVar {}

/// A material variable.
#[derive(Debug)]
#[repr(transparent)]
pub struct MaterialVar(NonNull<RawMaterialVar>);

impl MaterialVar {
    pub const fn from_raw(raw: *mut RawMaterialVar) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawMaterialVar) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawMaterialVar {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn set_tint(&self, r: f32, g: f32, b: f32) {
        type SetRgb = unsafe extern "C" fn(this: *const RawMaterialVar, r: f32, g: f32, b: f32);

        unsafe {
            virt::get::<SetRgb>(self.virtual_table(), 12 * 8)(self.as_ptr(), r, g, b);
        }
    }
}

extern "C" {
    /// Raw handle to a material.
    pub type RawMaterial;
}

unsafe impl Send for RawMaterial {}
unsafe impl Sync for RawMaterial {}

/// A material.
#[derive(Debug)]
#[repr(transparent)]
pub struct Material(NonNull<RawMaterial>);

impl Material {
    pub const fn from_raw(raw: *mut RawMaterial) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawMaterial) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawMaterial {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn var(&self, name: *const i8) -> Option<MaterialVar> {
        let mut found = false;
        let var = unsafe { self.var_unchecked(name, &mut found, true) };

        if found {
            MaterialVar::from_raw(var)
        } else {
            None
        }
    }

    pub unsafe fn var_unchecked(
        &self,
        name: *const i8,
        found: &mut bool,
        complain: bool,
    ) -> *mut RawMaterialVar {
        type Var = unsafe extern "C" fn(
            this: *const RawMaterial,
            name: *const i8,
            found: *mut bool,
            complain: bool,
        ) -> *mut RawMaterialVar;

        virt::get::<Var>(self.virtual_table(), 11 * 8)(self.as_ptr(), name, found, complain)
    }

    pub fn set_alpha(&self, a: f32) {
        type SetRgb = unsafe extern "C" fn(this: *const RawMaterial, a: f32);

        unsafe {
            virt::get::<SetRgb>(self.virtual_table(), 27 * 8)(self.as_ptr(), a);
        }
    }

    pub fn set_rgb(&self, r: f32, g: f32, b: f32) {
        type SetRgb = unsafe extern "C" fn(this: *const RawMaterial, r: f32, g: f32, b: f32);

        unsafe {
            virt::get::<SetRgb>(self.virtual_table(), 28 * 8)(self.as_ptr(), r, g, b);
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
        type SetFlag = unsafe extern "C" fn(this: *const RawMaterial, flag: i32, enabled: bool);

        unsafe {
            virt::get::<SetFlag>(self.virtual_table(), 29 * 8)(self.as_ptr(), flag, enabled);
        }
    }

    pub fn set_ignore_z(&self, enabled: bool) {
        self.set_flag(1 << 15, enabled);
    }

    pub fn set_wireframe(&self, enabled: bool) {
        self.set_flag(1 << 28, enabled);
    }

    pub fn get_alpha(&self) -> f32 {
        type GetAlpha = unsafe extern "C" fn(this: *const RawMaterial, alpha: *mut f32);

        let mut alpha = 0.0;

        unsafe {
            virt::get::<GetAlpha>(self.virtual_table(), 44 * 8)(self.as_ptr(), &mut alpha);
        }

        alpha
    }

    pub fn get_rgb(&self) -> [f32; 3] {
        type GetRgb =
            unsafe extern "C" fn(this: *const RawMaterial, r: *mut f32, g: *mut f32, b: *mut f32);

        let mut rgb = [0.0; 3];

        unsafe {
            virt::get::<GetRgb>(self.virtual_table(), 45 * 8)(
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

extern "C" {
    /// Raw handle to a material.
    pub type RawMaterialSystem;
}

unsafe impl Send for RawMaterialSystem {}
unsafe impl Sync for RawMaterialSystem {}

/// A material.
#[derive(Debug)]
#[repr(transparent)]
pub struct MaterialSystem(NonNull<RawMaterialSystem>);

impl MaterialSystem {
    pub const fn from_raw(raw: *mut RawMaterialSystem) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawMaterialSystem) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawMaterialSystem {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn create(&self, name: *const i8, settings: *const ()) {
        type Create = unsafe extern "C" fn(
            this: *const RawMaterialSystem,
            name: *const i8,
            settings: *const (),
        );

        unsafe {
            virt::get::<Create>(self.virtual_table(), 83 * 8)(self.as_ptr(), name, settings);
        }
    }

    pub fn find(
        &self,
        name: *const i8,
        texture_group_name: *const i8,
        complain: bool,
        complain_prefix: *const i8,
    ) -> Option<Material> {
        type Find = unsafe extern "C" fn(
            this: *const RawMaterialSystem,
            name: *const i8,
            texture_group_name: *const i8,
            complain: bool,
            complain_prefix: *const i8,
        ) -> *mut RawMaterial;

        unsafe {
            let raw = virt::get::<Find>(self.virtual_table(), 84 * 8)(
                self.as_ptr(),
                name,
                texture_group_name,
                complain,
                complain_prefix,
            );

            Material::from_raw(raw)
        }
    }
}
