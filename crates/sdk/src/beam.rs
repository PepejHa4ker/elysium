use super::Pad;
use elysium_math::Vec3;
use frosting::ffi::vtable;

#[repr(C)]
pub struct BeamInfo {
    pub kind: i32,
    pub start_entity: *const (),
    pub start_attachment: i32,
    pub end_entity: *const (),
    pub end_attachment: i32,
    pub start: Vec3,
    pub end: Vec3,
    pub model_index: i32,
    pub model_name: *const u8,
    pub halo_index: i32,
    pub halo_name: *const u8,
    pub halo_scale: f32,
    pub life: f32,
    pub width: f32,
    pub end_width: f32,
    pub fade_length: f32,
    pub amplitude: f32,
    pub brightness: f32,
    pub speed: f32,
    pub start_frame: f32,
    pub frame_rate: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub renderable: bool,
    pub segments: i32,
    pub flags: i32,
    pub ring_center: Vec3,
    pub ring_start_radius: f32,
    pub ring_end_radius: f32,
}

#[repr(C)]
pub struct Beam {
    _pad0: Pad<76>,
    flags: i32,
    _pad1: Pad<144>,
    die_at: f32,
}

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<9>,
    create_beam_points: unsafe extern "C" fn(
        this: *const ViewRenderBeams,
        beam_info: *const BeamInfo,
    ) -> *const Beam,
}

#[repr(C)]
pub struct ViewRenderBeams {
    vtable: &'static VTable,
}

impl ViewRenderBeams {
    #[inline]
    pub fn create_beam_points(&self, beam_info: &BeamInfo) -> *const Beam {
        unsafe { (self.vtable.create_beam_points)(self, beam_info) }
    }
}
