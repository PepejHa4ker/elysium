use super::{Entity, WeaponInfo};
use crate::global::Global;
use crate::managed::handle;

/// A weapon.
#[derive(Debug)]
#[repr(transparent)]
pub struct Weapon(Entity);

impl Weapon {
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

    /// Gets the amount of ammo remaining in this weapon.
    pub fn magazine(&self) -> Option<u32> {
        let magazine = unsafe {
            self.relative_entry::<i32>(Global::handle().networked().base_weapon.magazine)
        };

        if magazine < 0 {
            None
        } else {
            Some(magazine as u32)
        }
    }

    /// Gets the time that this weapon can shoot again in.
    pub fn next_attack_available_after(&self) -> f32 {
        unsafe {
            self.relative_entry(
                Global::handle()
                    .networked()
                    .base_weapon
                    .next_attack_available_after,
            )
        }
    }

    /// Gets the revolver cock time.
    pub fn revolver_cock_time(&self) -> Option<f32> {
        let time =
            unsafe { self.relative_entry(Global::handle().networked().weapon.revolver_cock_time) };

        if time >= 3.4028235e+38 {
            None
        } else {
            Some(time)
        }
    }

    fn actual_spread(&self) -> f32 {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> f32;

        unsafe { self.virtual_entry::<Fn>(521)(self.as_ptr()) }
    }

    fn actual_inaccuracy(&self) -> f32 {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> f32;

        unsafe { self.virtual_entry::<Fn>(521)(self.as_ptr()) }
    }

    /// Gets the item index of this weapon.
    pub fn item_index(&self) -> i32 {
        unsafe {
            let index: i32 = self.relative_entry(Global::handle().networked().item.index);

            // Strip skin bits, etc.
            index & 0xFFF
        }
    }

    /// Gets the weapons radius of spread (in radians).
    pub fn spread(&self) -> f32 {
        // spread + inaccuracy
        self.actual_spread() + self.actual_inaccuracy()
    }

    /// Gets the weapons radius of spread (in degree).
    pub fn spread_degrees(&self) -> f32 {
        self.spread().to_degrees()
    }

    /// Get's the weapon's information.
    fn info(&self) -> Option<WeaponInfo> {
        type Fn = unsafe extern "C" fn(this: *const handle::Entity) -> *mut handle::Entity;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(529)(self.as_ptr());

            WeaponInfo::new(ptr)
        }
    }

    /// Damage this weapon does.
    pub fn damage(&self) -> f32 {
        match self.info() {
            Some(info) => info.damage(),
            None => 0.0,
        }
    }

    /// How much damage this weapon does to entities with armor.
    pub fn weapon_armor_ratio(&self) -> f32 {
        match self.info() {
            Some(info) => info.weapon_armor_ratio(),
            None => 0.0,
        }
    }

    /// The amount of penetration (e.g. through walls) this weapon can do.
    pub fn penetration(&self) -> f32 {
        match self.info() {
            Some(info) => info.penetration(),
            None => 0.0,
        }
    }

    /// Maximum range for this weapon.
    pub fn range(&self) -> f32 {
        match self.info() {
            Some(info) => info.range(),
            None => 0.0,
        }
    }

    /// Range modifier applied to spread and damage. Reduces accuracy and damage the farther an
    /// object is.
    pub fn range_modifier(&self) -> f32 {
        match self.info() {
            Some(info) => info.range_modifier(),
            None => 0.0,
        }
    }
}
