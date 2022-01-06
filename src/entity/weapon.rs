use crate::netvars::Netvar;
use core::ptr::NonNull;

extern "C" {
    /// Raw handle to a weapon.
    pub type RawWeapon;
}

unsafe impl Send for RawWeapon {}
unsafe impl Sync for RawWeapon {}

/// A weapon.
#[derive(Debug)]
#[repr(transparent)]
pub struct Weapon(NonNull<RawWeapon>);

impl Weapon {
    pub(crate) const NETWORKED_CLASS_NAME: &'static str = "DT_WeaponCSBase";
    pub(crate) const NETWORKED_BASE_CLASS_NAME: &'static str = "DT_BaseCombatWeapon";

    #[inline]
    pub(crate) const fn from_raw(raw: *mut RawWeapon) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    #[inline]
    pub(crate) const unsafe fn from_raw_unchecked(raw: *mut RawWeapon) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *const RawWeapon {
        self.0.as_ptr()
    }

    #[inline]
    pub(crate) const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    #[inline]
    pub(crate) unsafe fn get(&self, offset: usize) -> *const u8 {
        (self.as_ptr() as *const u8).add(offset)
    }

    /// Gets the amount of ammo remaining in this weapon.
    #[inline]
    pub fn ammo(&self) -> Option<u32> {
        let ammo = *self.netvar::<i32>(Weapon::NETWORKED_BASE_CLASS_NAME, "m_iClip1");

        if ammo < 0 {
            None
        } else {
            Some(ammo as u32)
        }
    }

    /// Gets the time that this weapon can shoot again in.
    #[inline]
    pub fn next_attack_available_after(&self) -> f32 {
        *self.netvar(Weapon::NETWORKED_BASE_CLASS_NAME, "m_flNextPrimaryAttack")
    }

    /// Gets the revolver cock time.
    #[inline]
    pub fn revolver_cock_time(&self) -> Option<f32> {
        let time = *self.netvar::<f32>(Weapon::NETWORKED_CLASS_NAME, "m_flPostponeFireReadyTime");

        if time >= 3.4028235e+38 {
            None
        } else {
            Some(time)
        }
    }

    /// Is this weapon being reloaded currently.
    #[inline]
    pub fn in_reload(&self) -> bool {
        !*self.netvar::<bool>(Weapon::NETWORKED_BASE_CLASS_NAME, "m_bInReload")
    }

    #[inline]
    fn raw_spread(&self) -> f32 {
        unsafe { *(self.get(0x521) as *const f32) }
    }

    #[inline]
    fn raw_inaccuracy(&self) -> f32 {
        unsafe { *(self.get(0x551) as *const f32) }
    }

    /// Gets the weapons radius of spread (in radians).
    #[inline]
    pub fn spread(&self) -> f32 {
        self.raw_spread() + self.raw_inaccuracy()
    }

    /// Gets the weapons radius of spread (in degree).
    #[inline]
    pub fn spread_degrees(&self) -> f32 {
        self.spread().to_degrees()
    }

    /// Get's the weapon's information.
    #[inline]
    fn info(&self) -> Option<WeaponInfo> {
        unsafe {
            type Info = unsafe extern "C" fn(this: *const RawWeapon) -> *mut RawWeaponInfo;

            let raw = virt::get::<Info>(self.virtual_table(), 529 * 8)(self.as_ptr());

            println!("{raw:?}");

            WeaponInfo::from_raw(raw)
        }
    }

    /// Damage this weapon does.
    #[inline]
    pub fn damage(&self) -> f32 {
        match self.info() {
            Some(info) => info.damage(),
            None => 0.0,
        }
    }

    /// How much damage this weapon does to entities with armor.
    #[inline]
    pub fn weapon_armor_ratio(&self) -> f32 {
        match self.info() {
            Some(info) => info.weapon_armor_ratio(),
            None => 0.0,
        }
    }

    /// The amount of penetration (e.g. through walls) this weapon can do.
    #[inline]
    pub fn penetration(&self) -> f32 {
        match self.info() {
            Some(info) => info.penetration(),
            None => 0.0,
        }
    }

    /// Maximum range for this weapon.
    #[inline]
    pub fn range(&self) -> f32 {
        match self.info() {
            Some(info) => info.range(),
            None => 0.0,
        }
    }

    /// Range modifier applied to spread and damage. Reduces accuracy and damage the farther an
    /// object is.
    #[inline]
    pub fn range_modifier(&self) -> f32 {
        match self.info() {
            Some(info) => info.range_modifier(),
            None => 0.0,
        }
    }
}

impl Netvar for Weapon {
    fn as_ptr(&self) -> *const () {
        Weapon::as_ptr(self) as _
    }
}

extern "C" {
    /// Raw handle to a weapon's info.
    pub type RawWeaponInfo;
}

unsafe impl Send for RawWeaponInfo {}
unsafe impl Sync for RawWeaponInfo {}

/// A weapon's info.
#[derive(Debug)]
#[repr(transparent)]
pub struct WeaponInfo(NonNull<RawWeaponInfo>);

impl WeaponInfo {
    #[inline]
    pub(crate) const fn from_raw(raw: *mut RawWeaponInfo) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    #[inline]
    pub(crate) const unsafe fn from_raw_unchecked(raw: *mut RawWeaponInfo) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *const RawWeaponInfo {
        self.0.as_ptr()
    }

    #[inline]
    pub(crate) const fn virtual_table(&self) -> *const *const u8 {
        unsafe { *(self.as_ptr() as *const *const *const u8) }
    }

    #[inline]
    pub(crate) unsafe fn get(&self, offset: usize) -> *const u8 {
        (self.as_ptr() as *const u8).add(offset)
    }

    /// Damage this weapon does.
    #[inline]
    pub fn damage(&self) -> f32 {
        unsafe { *(self.get(0x16C) as *const i32) as f32 }
    }

    /// How much damage this weapon does to entities with armor.
    #[inline]
    pub fn weapon_armor_ratio(&self) -> f32 {
        unsafe { *(self.get(0x174) as *const f32) }
    }

    /// The amount of penetration (e.g. through walls) this weapon can do.
    #[inline]
    pub fn penetration(&self) -> f32 {
        unsafe { *(self.get(0x17C) as *const f32) }
    }

    /// Maximum range for this weapon.
    #[inline]
    pub fn range(&self) -> f32 {
        unsafe { *(self.get(0x188) as *const f32) }
    }

    /// Range modifier applied to spread and damage. Reduces accuracy and damage the farther an
    /// object is.
    #[inline]
    pub fn range_modifier(&self) -> f32 {
        unsafe { *(self.get(0x18C) as *const f32) }
    }
}
