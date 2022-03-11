use crate::client::Class;
use crate::global::Global;
use crate::managed::{handle, Managed};
use crate::mem;
use crate::model::Model;
use core::cmp;
use providence_math::{Matrix3x4, Vec3};

pub use fog::Fog;
pub use id::EntityId;
pub use list::EntityList;
pub use player::Player;
pub use weapon::Weapon;
pub use weapon_info::WeaponInfo;

mod fog;
mod id;
mod list;
mod player;
mod weapon;
mod weapon_info;

/// An entity.
#[derive(Debug)]
#[repr(transparent)]
pub struct Entity(Managed<handle::Entity>);

impl Entity {
    pub fn new(ptr: *mut handle::Entity) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Entity) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Entity {
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

    /// Returns a pointer to the entity's networkable.
    pub unsafe fn networkable(&self) -> *const () {
        self.relative_offset(16)
    }

    /// Returns a pointer to the first element in the entity's networkable virtual table.
    pub unsafe fn networkable_virtual_table(&self) -> *const () {
        mem::virtual_table(self.networkable())
    }

    /// Returns a pointer to the object at `offset` in the entity's networkable virtual table.
    pub unsafe fn networkable_virtual_offset(&self, offset: usize) -> *const () {
        mem::virtual_offset(self.networkable(), offset)
    }

    /// Returns an object at `offset` in the entity's networkable virtual table.
    pub unsafe fn networkable_virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        mem::virtual_entry(self.networkable(), offset)
    }

    /// Returns a pointer to the entity's renderable.
    pub unsafe fn renderable(&self) -> *const () {
        self.relative_offset(8)
    }

    /// Returns a pointer to the entity's renderable virtual table.
    pub unsafe fn renderable_virtual_table(&self) -> *const () {
        mem::virtual_table(self.renderable())
    }

    /// Returns a pointer to the object at `offset` in the entity's renderable virtual table.
    pub unsafe fn renderable_virtual_offset(&self, offset: usize) -> *const () {
        mem::virtual_offset(self.renderable(), offset)
    }

    /// Returns an object at `offset` in the entity's renderable virtual table.
    pub unsafe fn renderable_virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        mem::virtual_entry(self.renderable(), offset)
    }

    /// Is this entity dormant.
    pub fn class(&self) -> *const Class {
        type Fn = unsafe extern "C" fn(this: *const ()) -> *const Class;

        unsafe { self.networkable_virtual_entry::<Fn>(2)(self.networkable()) }
    }

    /// Is this entity dormant.
    pub fn is_dormant(&self) -> bool {
        type Fn = unsafe extern "C" fn(this: *const ()) -> bool;

        unsafe { self.networkable_virtual_entry::<Fn>(9)(self.networkable()) }
    }

    /// Index of this entity in the engine
    pub fn index(&self) -> i32 {
        type Fn = unsafe extern "C" fn(this: *const ()) -> i32;

        unsafe { self.networkable_virtual_entry::<Fn>(10)(self.networkable()) }
    }

    /// Model of this entity.
    pub fn model(&self) -> Option<&Model> {
        type Fn = unsafe extern "C" fn(this: *const ()) -> *const Model;

        unsafe {
            let ptr = self.renderable_virtual_entry::<Fn>(8)(self.renderable());

            if ptr.is_null() {
                None
            } else {
                Some(&*ptr)
            }
        }
    }

    pub fn setup_bones(
        &self,
        bone_matrix: *mut Matrix3x4,
        max_bones: i32,
        bone_mask: i32,
        current_time: f32,
    ) -> bool {
        type Fn = unsafe extern "C" fn(
            this: *const (),
            bone_matrix: *mut Matrix3x4,
            max_bones: i32,
            bone_mask: i32,
            current_time: f32,
        ) -> bool;

        unsafe {
            self.renderable_virtual_entry::<Fn>(13)(
                self.renderable(),
                bone_matrix,
                max_bones,
                bone_mask,
                current_time,
            )
        }
    }

    pub fn origin_ptr(&self) -> *mut Vec3 {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> *mut Vec3;

        unsafe { self.virtual_entry::<Fn>(12)(self.as_ptr()) }
    }

    pub fn origin(&self) -> Vec3 {
        unsafe { *self.origin_ptr() }
    }

    /// Entity's movement kind.
    pub fn move_kind(&self) -> i32 {
        unsafe { self.relative_entry(Global::handle().networked().base_entity.render_mode + 1) }
    }

    pub fn team(&self) -> &mut i32 {
        unsafe {
            &mut *(self.relative_offset(Global::handle().networked().base_entity.team) as *mut i32)
        }
    }

    /// If this entity is on a ladder.
    pub fn on_ladder(&self) -> bool {
        self.move_kind() == 9
    }

    /// Is this entity noclipping.
    pub fn is_noclip(&self) -> bool {
        self.move_kind() == 8
    }

    pub fn is_player(&self) -> bool {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> bool;

        unsafe { self.virtual_entry::<Fn>(210)(self.as_ptr()) }
    }
}

impl cmp::PartialEq<Entity> for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.index() == other.index()
    }
}

impl cmp::PartialEq<Player> for Entity {
    fn eq(&self, other: &Player) -> bool {
        self.index() == other.index()
    }
}
