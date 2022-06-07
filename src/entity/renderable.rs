    use crate::vtable_validate;

    #[repr(C)]
    struct VTable {
        _pad0: vtable::Pad<5>,
        should_draw: unsafe extern "C" fn(this: *const Renderable) -> bool,
        _pad0: vtable::Pad<2>,
        get_model: unsafe extern "C" fn(this: *const Renderable) -> *const u8,
        _pad1: vtable::Pad<3>,
        setup_bones: unsafe extern "C" fn(
            this: *const Renderable,
            bones: *mut u8,
            max_bones: i32,
            bone_mask: i32,
            current_time: f32,
        ) -> bool,
    }

    vtable_validate! {
        should_draw => 5,
        get_model => 8,
        setup_bones => 13,
    }

    #[repr(C)]
    pub struct Renderable {
        vtable: &'static VTable,
    }
