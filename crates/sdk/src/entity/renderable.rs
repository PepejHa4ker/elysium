use crate::vtable_validate;
use elysium_math::Matrix3x4;
use frosting::ffi::vtable;

#[derive(Debug)]
#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<5>,
    should_draw: unsafe extern "C" fn(this: *const Renderable) -> bool,
    _pad1: vtable::Pad<2>,
    get_model: unsafe extern "C" fn(this: *const Renderable) -> *const u8,
    _pad2: vtable::Pad<4>,
    setup_bones: unsafe extern "C" fn(
        this: *const Renderable,
        bones: *mut Matrix3x4,
        len: i32,
        mask: i32,
        time: f32,
    ) -> bool,
}

vtable_validate! {
    should_draw => 5,
    get_model => 8,
    setup_bones => 13,
}

#[derive(Debug)]
#[repr(C)]
pub struct Renderable {
    vtable: &'static VTable,
}

impl Renderable {
    #[inline]
    pub fn should_draw(&self) -> bool {
        unsafe { (self.vtable.should_draw)(self) }
    }

    #[inline]
    pub fn model(&self) -> *const u8 {
        unsafe { (self.vtable.get_model)(self) }
    }

    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        println!("{mask:?} {time:?}");
        unsafe {
            (self.vtable.setup_bones)(self, bones.as_mut_ptr(), bones.len() as i32, mask, time)
        }
    }
}
