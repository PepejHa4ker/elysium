use crate::global::Global;
use crate::material::Material;
use crate::model::{DrawModelState, ModelRenderInfo};
use nalgebra::{Rotation3, Unit};
use nalgebra_glm::{Mat3x4, TVec};
use palette::{FromColor, Hsl, Hue, Lch, Pixel, Srgb};
use sdk::{Matrix3x4, Vec3};

pub type Signature = unsafe extern "C" fn(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
);

pub unsafe extern "C" fn do_cham(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
    global: &Global,
    material: &Material,
) {
    material.set_rgba8(0x12, 0x12, 0x12, 0xFF);

    global.model_render().set_material(&material);
    global.draw_model_execute_original(this, context, state, info, bone_to_world);

    // rainbow, also aids code
    let srgb: [u8; 3] = [0xFF, 0x00, 0x00];
    let srgb: Srgb<u8> = *Srgb::from_raw(&srgb);
    let srgb: Srgb<f32> = srgb.into_format();

    let hsl = Hsl::from_color(srgb);
    let hsl = hsl.shift_hue(global.globals().current_time * 50.0);

    let srgb: Srgb<f32> = Srgb::from_color(hsl);
    let srgb: Srgb<u8> = srgb.into_format();
    let srgb: [u8; 3] = srgb.into_raw();

    material.set_rgba8(srgb[0], srgb[1], srgb[2], 0xFF);
    material.set_wireframe(true);

    global.model_render().set_material(&material);
    global.draw_model_execute_original(this, context, state, info, bone_to_world);
    global.model_render().reset_material();

    // reset for flat chams above next call
    material.set_wireframe(false);
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
    let material = global.flat_material();

    if name.starts_with("models/player") {
        if !name.contains("contactshadow") {
            do_cham(
                this,
                context,
                state,
                info,
                bone_to_world,
                &global,
                &material,
            );
        }
    } else if name.starts_with("models/weapons/v_") {
        do_cham(
            this,
            context,
            state,
            info,
            bone_to_world,
            &global,
            &material,
        );
    } else if name.starts_with("weapons") {
        do_cham(
            this,
            context,
            state,
            info,
            bone_to_world,
            &global,
            &material,
        );
    } else {
        global.draw_model_execute_original(this, context, state, info, bone_to_world);
    }
}
