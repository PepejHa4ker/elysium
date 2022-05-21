use crate::managed::{handle, Managed};
use providence_util::virtual_table;
use sdk2::Pad;

#[derive(Debug)]
#[repr(C)]
pub struct SurfacePhysics {
    pub friction: f32,
    pub elasticity: f32,
    pub density: f32,
    pub thickness: f32,
    pub dampening: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct SurfaceAudio {
    pub audio_reflectivity: f32,
    pub audio_hardness_factor: f32,
    pub audio_roughness_factor: f32,
    pub scrape_rough_threshold: f32,
    pub impact_hard_threshold: f32,
    pub audio_hard_min_velocity: f32,
    pub high_pitch_occlusion: f32,
    pub mid_pitch_occlusion: f32,
    pub low_pitch_occlusion: f32,
}

#[derive(Debug)]
#[repr(C)]
pub struct SurfaceSounds {
    pub walk_left: i16,
    pub walk_right: i16,
    pub run_left: i16,
    pub run_right: i16,
    pub impact_soft: i16,
    pub impact_hard: i16,
    pub scrape_smooth: i16,
    pub scrape_rough: i16,
    pub bullet_impact: i16,
    pub rolling: i16,
    pub break_sound: i16,
    pub strain: i16,
}

#[derive(Debug)]
#[repr(C)]
pub struct SurfaceProperties {
    pub max_speed_factor: f32,
    pub jump_jactor: f32,
    pub penetration_modifier: f32,
    pub damage_modifier: f32,
    pub material: u16,
    pub climbable: u8,
}

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    pub physics: SurfacePhysics,
    pub audio: SurfaceAudio,
    pub sounds: SurfaceSounds,
    pub properties: SurfaceProperties,
    pub pad: Pad<48>,
}

/// Physics.
#[derive(Debug)]
#[repr(transparent)]
pub struct Physics(Managed<handle::Physics>);

impl Physics {
    pub fn new(ptr: *mut handle::Physics) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Physics) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Physics {
        self.0.as_ptr()
    }

    virtual_table! {
        fn query_unchecked[6](index: i32) -> *const Surface;
    }

    pub fn query(&self, index: i32) -> Option<Surface> {
        unsafe {
            let ptr = self.query_unchecked(index);

            if ptr.is_null() {
                None
            } else {
                Some(ptr.read())
            }
        }
    }
}
