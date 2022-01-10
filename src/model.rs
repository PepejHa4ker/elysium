use crate::managed::{handle, Managed};
use crate::material::Material;
use core::marker::PhantomData;
use core::ptr;
use sdk::{Angle, Matrix3x4, Pad, Vector};
use spirit::Str;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
#[repr(i32)]
#[rustfmt::skip] // rustfmt does an ugly
pub enum UsedBy {
    Anything    = 0x00_0F_FF_00,
    Hitbox      = 0x00_00_01_00, // bone (or child) used by a hitbox
    Attachment  = 0x00_00_02_00, // bone (or child) used by an attachment point
    VertexMask  = 0x00_03_FC_00,
    VertexLoD0  = 0x00_00_04_00, // bone (or child) used by the top-level model via skinned vertex
    VertexLoD1  = 0x00_00_08_00,
    VertexLoD2  = 0x00_00_10_00,
    VertexLoD3  = 0x00_00_20_00,
    VertexLoD4  = 0x00_00_40_00,
    VertexLoD5  = 0x00_00_80_00,
    VertexLoD6  = 0x00_01_00_00,
    VertexLoD7  = 0x00_02_00_00,
    BoneMerge   = 0x00_04_00_00, // bone is available for bone merge to occur
    AlwaysSetup = 0x00_08_00_00,
}

#[derive(Debug)]
#[repr(C)]
pub struct MagicArray<T> {
    len: i32,
    offset: i32,
    phantom: PhantomData<T>,
}

impl<T> MagicArray<T> {
    pub const fn len(&self) -> i32 {
        self.len
    }

    pub const fn offset(&self) -> i32 {
        self.offset
    }

    pub unsafe fn get_unchecked(&self, base_address: *const u8, index: i32) -> *const T {
        base_address
            .offset(self.offset as isize)
            .offset(index as isize)
            .cast()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Bone {
    pub name_offset: i32,
    pub parent: i32,
    pub bone_controller: [i32; 6],
    pub position: Vector,
    pub quaternion: [f32; 4],
    pub rotation: [f32; 3],
    pub position_scale: Vector,
    pub rotation_scale: Vector,
    pub position_to_bone: Matrix3x4,
    pub quaternion_alignment: [f32; 4],
    pub flags: i32,
    pub procedural_kind: i32,
    pub procedural_offset: i32,
    pub physics_bone: i32,
    pub surface_prop_offset: i32,
    pub contents: i32,
    pub surface_prop_lookup: i32,
    _pad0: Pad<28>,
}

impl Bone {
    pub const fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    fn name<'a>(&'a self) -> &'a Str {
        unsafe { Str::new(self.as_ptr().offset(self.name_offset as isize).cast()) }
    }

    unsafe fn procedural(&self) -> *const () {
        match self.procedural_offset {
            0 => ptr::null(),
            offset => self.as_ptr().offset(offset as isize).cast(),
        }
    }

    unsafe fn get_surface_prop(&self) -> *const i8 {
        self.as_ptr()
            .offset(self.surface_prop_offset as isize)
            .cast()
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct BoundingBox {
    pub bone: i32,
    pub group: i32,
    pub max: Vector,
    pub min: Vector,
    pub hitbox_name_offset: i32,
    _pad0: Pad<12>,
    pub radius: f32,
    _pad1: Pad<16>,
}

impl BoundingBox {
    pub const fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    pub fn name<'a>(&'a self) -> Option<&'a Str> {
        unsafe {
            match self.hitbox_name_offset {
                0 => None,
                offset => Some(Str::new(self.as_ptr().offset(offset as isize).cast())),
            }
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct HitboxSet {
    pub name_offset: i32,
    pub hitboxes: MagicArray<BoundingBox>,
}

impl HitboxSet {
    pub const fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    pub fn name<'a>(&'a self) -> &'a Str {
        unsafe { Str::new(self.as_ptr().offset(self.name_offset as isize).cast()) }
    }

    pub unsafe fn hitbox_unchecked(&self, index: i32) -> *const BoundingBox {
        self.hitboxes.get_unchecked(self.as_ptr(), index)
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Hdr {
    pub id: i32,
    pub version: i32,
    pub checksum: i32,
    pub name: [i8; 64],
    pub length: i32,
    pub eye_position: Vector,
    pub illumination_position: Vector,
    pub hull_min: Vector,
    pub hull_max: Vector,
    pub view_bounding_box_min: Vector,
    pub view_bounding_box_max: Vector,
    pub flags: i32,
    pub bones: MagicArray<Bone>,
    pub bone_controllers: MagicArray<()>,
    pub hitbox_sets: MagicArray<HitboxSet>,
    pub local_anims: MagicArray<()>,
    pub local_seqs: MagicArray<()>,
    pub textures: MagicArray<()>,
    pub raw_textures: MagicArray<()>,
    pub replacable_textures: MagicArray<()>,
    pub body_parts: MagicArray<()>,
    pub local_attachments: MagicArray<()>,
    pub local_nodes: MagicArray<()>,
    pub flex_desc: MagicArray<()>,
    pub flex_controllers: MagicArray<()>,
    pub flex_rules: MagicArray<()>,
    pub ik_chains: MagicArray<()>,
    pub mouths: MagicArray<()>,
    pub local_pose_parameters: MagicArray<()>,
    pub surface_pos_offset: i32,
    pub key_values: MagicArray<()>,
    pub local_ik_autoplaylocks: MagicArray<()>,
    pub mass: f32,
    pub contents: i32,
    pub include_models: MagicArray<()>,
    pub virtual_model: *mut (),
    pub animation_block_name_offset: i32,
    pub animation_blocks: MagicArray<()>,
    pub bone_table_by_name_index: i32,
    pub vertex_base: *const (),
    pub index_base: *const (),
    pub constant_directional_light_dot: u8,
    pub root_lod: u8,
    pub allowed_root_lods: u8,
    _pad0: Pad<5>,
    pub flex_controller_ui: MagicArray<()>,
    _pad1: Pad<16>,
}

/// Model info interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct ModelInfo(Managed<handle::ModelInfo>);

impl ModelInfo {
    pub fn new(ptr: *mut handle::ModelInfo) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::ModelInfo) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::ModelInfo {
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

    pub fn index_of(&self, filename: *const i8) -> i32 {
        type Fn = unsafe extern "C" fn(this: *const handle::ModelInfo, filename: *const i8) -> i32;

        unsafe { self.virtual_entry::<Fn>(3)(self.as_ptr(), filename) }
    }

    pub fn name_of<'a>(&'a self, model: &Model) -> &'a Str {
        type Fn =
            unsafe extern "C" fn(this: *const handle::ModelInfo, model: *const Model) -> *const u8;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(4)(self.as_ptr(), model);

            Str::new(ptr)
        }
    }

    pub fn studio_model_of(&self, model: &Model) -> *const Hdr {
        type Fn =
            unsafe extern "C" fn(this: *const handle::ModelInfo, model: *const Model) -> *const Hdr;

        unsafe { self.virtual_entry::<Fn>(31)(self.as_ptr(), model) }
    }
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct DrawModelState {
    pub studio: *const (),
    pub hardware_data: *const (),
    pub renderable: *const (),
    pub model_to_world: *const Matrix3x4,
    pub decals: *const (),
    pub draw_flags: i32,
    pub lod: i32,
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct Model {
    pub name: [u8; 255],
}

#[derive(Debug)]
#[non_exhaustive]
#[repr(C)]
pub struct ModelRenderInfo {
    pub origin: Vector,
    pub angles: Angle,
    _pad0: Pad<4>,
    pub renderable: *const *const (),
    pub model: *const Model,
    pub model_to_world: *const Matrix3x4,
    pub lighting_offset: *const Matrix3x4,
    pub lighting_origin: *const Vector,
    pub flags: i32,
    pub entity_index: i32,
    pub skin: i32,
    pub body: i32,
    pub hitboxset: i32,
    pub instance: *const (),
}

/// Model renderer interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct ModelRender(Managed<handle::ModelRender>);

impl ModelRender {
    pub fn new(ptr: *mut handle::ModelRender) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::ModelRender) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::ModelRender {
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

    pub fn set_material(&self, material: &Material) {
        unsafe {
            self.material_override_unchecked(material.as_ptr());
        }
    }

    pub fn reset_material(&self) {
        unsafe { self.material_override_unchecked(ptr::null::<()>() as *const handle::Material) }
    }

    pub unsafe fn material_override_unchecked(&self, material: *const handle::Material) {
        type Fn = unsafe extern "C" fn(
            this: *const handle::ModelRender,
            material: *const handle::Material,
        );

        self.virtual_entry::<Fn>(1)(self.as_ptr(), material)
    }

    pub fn draw_model(
        &self,
        context: *const (),
        state: &DrawModelState,
        info: &ModelRenderInfo,
        bone_to_world: &Matrix3x4,
    ) {
        type DrawModel = unsafe extern "C" fn(
            this: *const handle::ModelRender,
            context: *const (),
            state: *const DrawModelState,
            info: *const ModelRenderInfo,
            bone_to_world: *const Matrix3x4,
        );

        unsafe {
            self.virtual_entry::<DrawModel>(21)(self.as_ptr(), context, state, info, bone_to_world)
        }
    }
}
