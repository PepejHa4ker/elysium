use crate::{state, Networked};
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::entity::{EntityId, Networkable, Renderable};
use elysium_sdk::{object_validate, vtable_validate};
use frosting::ffi::vtable;

#[derive(Debug)]
#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<12>,
    origin: unsafe extern "C" fn(this: *const Entity) -> *const Vec3,
    _pad1: vtable::Pad<144>,
    is_player: unsafe extern "C" fn(this: *const Entity) -> bool,
}

vtable_validate! {
    origin => 12,
    is_player => 157,
}

#[derive(Debug)]
#[repr(C)]
pub struct Entity {
    vtable: &'static VTable,
    renderable: &'static Renderable,
    networkable: &'static Networkable,
}

object_validate! {
    Entity;
    vtable => 0,
    renderable => 8,
    networkable => 16,
}

impl Entity {
    /// is the entity dormant
    #[inline]
    pub fn is_dormant(&self) -> bool {
        self.networkable.is_dormant()
    }

    /// the entity's index
    #[inline]
    pub fn index(&self) -> i32 {
        self.networkable.index()
    }

    /// the entity's model
    #[inline]
    pub fn model(&self) -> *const u8 {
        self.renderable.model()
    }

    /// setup bones
    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        self.renderable.setup_bones(bones, mask, time)
    }

    /// should draw?
    #[inline]
    pub fn should_draw(&self) -> bool {
        self.renderable.should_draw()
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        unsafe { *(self.vtable.origin)(self) }
    }

    #[inline]
    pub fn is_player(&self) -> bool {
        unsafe { (self.vtable.is_player)(self) }
    }

    /// only for base_players
    fn is_dead_ptr(&self) -> *const u8 {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            this.byte_add(networked.base_player.is_dead)
        }
    }

    /// only for base_players
    pub fn view_angle(&self) -> &mut Vec3 {
        unsafe { &mut *self.is_dead_ptr().byte_add(4).as_mut().cast() }
    }
}
