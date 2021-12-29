use crate::global::Global;
use crate::model::{DrawModelState, ModelRenderInfo};
use sdk::Matrix3x4;

pub type Signature = unsafe extern "C" fn(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
);

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
    let name = {
        let ptr = global.model_info().name_of(&*(*info).model);
        let cstr = std::ffi::CStr::from_ptr(ptr);

        cstr.to_string_lossy()
    };

    if name.starts_with("models/player") && !name.contains("contactshadow") {
        let material = global.flat_material();

        material.set_rgba(1.0, 1.0, 1.0, 0.6);

        global.model_render().material_override(&material);
        global.draw_model_execute_original(this, context, state, info, bone_to_world);
        global
            .model_render()
            .material_override_unchecked(core::ptr::null::<()>() as *const _);
    }
}
