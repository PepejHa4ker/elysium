use crate::{state, Networked};
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::entity::{Networkable, ObserverMode, Renderable};
use elysium_sdk::{object_validate, vtable_validate};
use frosting::ffi::vtable;

#[derive(Debug)]
#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<12>,
    origin: unsafe extern "C" fn(this: *const Entity) -> *const Vec3,
    _pad1: vtable::Pad<144>,
    is_player: unsafe extern "C" fn(this: *const Entity) -> bool,
    _pad2: vtable::Pad<199>,
    observer_mode: unsafe extern "C" fn(this: *const Entity) -> ObserverMode,
}

vtable_validate! {
    origin => 12,
    is_player => 157,
    observer_mode => 357,
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

    /// only for base_entitys
    #[inline]
    fn render_mode_address(&self) -> *const u8 {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            this.byte_add(networked.base_entity.render_mode)
        }
    }

    #[inline]
    pub fn move_kind(&self) -> i32 {
        unsafe { *self.render_mode_address().byte_add(1).cast() }
    }

    /// only for base_players
    #[inline]
    unsafe fn is_dead_address(&self) -> *const u8 {
        let this = (self as *const Self).cast::<u8>();
        let networked = &*state::networked().cast::<Networked>();

        this.byte_add(networked.base_player.is_dead)
    }

    #[inline]
    unsafe fn view_angle_address(&self) -> *mut Vec3 {
        self.is_dead_address().byte_add(4).as_mut().cast()
    }

    /// only for base_players
    #[inline]
    pub fn view_angle(&self) -> &mut Vec3 {
        unsafe { &mut *self.view_angle_address() }
    }

    #[inline]
    pub fn set_view_angle(&self, angle: Vec3) {
        unsafe {
            self.view_angle_address().write(angle);
        }
    }

    /// only for base_players
    #[inline]
    pub fn velocity(&self) -> Vec3 {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            *this.byte_add(networked.base_player.velocity).cast()
        }
    }

    /// only for players
    #[inline]
    pub fn flags(&self) -> i32 {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            *this.byte_add(networked.player.flags).cast()
        }
    }

    #[inline]
    pub fn observer_mode(&self) -> ObserverMode {
        unsafe { (self.vtable.observer_mode)(self) }
    }

    /// only for players
    #[inline]
    pub fn armor(&self) -> i32 {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            *this.byte_add(networked.player.armor).cast()
        }
    }

    /// only for players
    #[inline]
    pub fn has_helmet(&self) -> bool {
        unsafe {
            let this = (self as *const Self).cast::<u8>();
            let networked = &*state::networked().cast::<Networked>();

            *this.byte_add(networked.player.has_helmet).cast()
        }
    }
}
