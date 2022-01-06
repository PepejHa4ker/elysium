use crate::global::Global;
use crate::model::{DrawModelState, ModelRenderInfo};
use nalgebra::{Rotation3, Unit};
use nalgebra_glm::{Mat3x4, TVec};
use sdk::{Matrix3x4, Vector};

pub type Signature = unsafe extern "C" fn(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
);

#[derive(Clone, Copy)]
pub struct BoneToWorld {
    bones: [Mat3x4; 128],
}

impl BoneToWorld {
    pub fn as_ptr(&self) -> *const Matrix3x4 {
        self.bones.as_ptr() as *const _
    }

    pub fn rotate_yaw(self, origin: Vector, yaw: f32) -> Self {
        let mut this = self;
        let yaw = yaw.to_radians();

        for bone in this.bones.iter_mut() {
            *bone = *bone
                * Rotation3::from_axis_angle(&Unit::new_normalize(TVec::z()), yaw).to_homogeneous();
        }

        this
    }

    /*pub fn translate(self, vector: Vector) -> Self {
        let mut this = self;

        for bone in this.bones.iter_mut() {
            *bone = bone.with_w(bone.w() + vector);
        }

        this
    }*/
}

pub unsafe extern "C" fn hook(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    if this.is_null()
        || context.is_null()
        || state.is_null()
        || info.is_null()
        || bone_to_world.is_null()
    {
        return;
    }

    if (*info).model.is_null() {
        return;
    }

    let global = Global::handle();
    let name = global.model_info().name_of(&*(*info).model);
    let name = name.as_str();

    if !name.contains("contactshadow") {
        if name.starts_with("models/player") {
            let material = global.flat_material();

            material.set_rgba8(0x67, 0xC8, 0x95, 0x90);
            material.set_ignore_z(true);

            global.model_render().set_material(&material);
            global.draw_model_execute_original(this, context, state, info, bone_to_world);
            global.model_render().reset_material();
            global.draw_model_execute_original(this, context, state, info, bone_to_world);

            let bone_to_world = *(bone_to_world as *const BoneToWorld);
            let bone_to_world = bone_to_world.rotate_yaw((*info).origin, 90.0);

            /*global.model_render().material_override(&material);
            global.draw_model_execute_original(this, context, state, info, bone_to_world.as_ptr());
            global
                .model_render()
                .material_override_unchecked(core::ptr::null::<()>() as *const _);*/
        }
    }
}
