use super::Entity;
use crate::managed::handle;

/// A weapon's information.
#[derive(Debug)]
#[repr(transparent)]
pub struct WeaponInfo(Entity);

impl WeaponInfo {
    pub fn new(ptr: *mut handle::Entity) -> Option<Self> {
        Some(Self(Entity::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Entity) -> Self {
        Self(Entity::new_unchecked(ptr))
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

    /// Damage this weapon does.
    pub fn damage(&self) -> f32 {
        unsafe { *(self.relative_offset(364) as *const i32) as f32 }
    }

    /// How much damage this weapon does to entities with armor.
    pub fn weapon_armor_ratio(&self) -> f32 {
        unsafe { *(self.relative_offset(372) as *const f32) }
    }

    /// The amount of penetration (e.g. through walls) this weapon can do.
    pub fn penetration(&self) -> f32 {
        unsafe { *(self.relative_offset(380) as *const f32) }
    }

    /// Maximum range for this weapon.
    pub fn range(&self) -> f32 {
        unsafe { *(self.relative_offset(392) as *const f32) }
    }

    /// Range modifier applied to spread and damage. Reduces accuracy and damage the farther an
    /// object is.
    pub fn range_modifier(&self) -> f32 {
        unsafe { *(self.relative_offset(396) as *const f32) }
    }
}
