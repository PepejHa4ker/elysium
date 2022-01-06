use core::ptr::NonNull;
use sdk::Pad;

extern "C" {
    /// Raw handle to physics.
    pub type RawPhysics;
}

unsafe impl Send for RawPhysics {}
unsafe impl Sync for RawPhysics {}

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    pub friction: f32,
    pub elasticity: f32,
    pub density: f32,
    pub thickness: f32,
    pub dampening: f32,
    _pad0: Pad<68>,
    pub penetration_modifier: f32,
    pub damage_modifier: f32,
    pub material: u16,
}

/// Physics.
#[derive(Debug)]
#[repr(transparent)]
pub struct Physics(NonNull<RawPhysics>);

impl Physics {
    pub const fn from_raw(raw: *mut RawPhysics) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawPhysics) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawPhysics {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn query(&self, index: i32) -> Option<Surface> {
        type Query = unsafe extern "C" fn(this: *const RawPhysics, index: i32) -> *const Surface;

        unsafe {
            let raw = virt::get::<Query>(self.virtual_table(), 5 * 8)(self.as_ptr(), index);

            if raw.is_null() {
                None
            } else {
                Some(raw.read())
            }
        }
    }
}
