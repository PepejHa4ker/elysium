use crate::global::Global;
use crate::model::{DrawModelState, ModelRenderInfo};
use providence_math::Matrix3x4;

pub type Signature = unsafe extern "C" fn(
    this: *const (),
    context: *const (),
    state: *const DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
);

pub unsafe extern "C" fn hook<'a>(
    this: *const (),
    context: *const (),
    state: &'a DrawModelState,
    info: &'a ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    //let entity_index = info.entity_index;

    // dont render non-players
    //if !matches!(entity_index, 1..=64) {
    //println!("Skip rendering entity {entity_index}: Not witnin 1..=64.");

    //return;
    //}

    let global = Global::handle();
    let flat_material = global.flat_material();

    //flat_material.set_ignore_z(false);
    //flat_material.set_wireframe(false);

    /*if let Some(entity) = global.entity_list().get(entity_index) {
        println!("{entity_index} -> {entity:?}");
    }

    // obtain local player index
    let local_player_index = global
        .local_player()
        .map(|player| player.index())
        .unwrap_or(-1);

    // skip local player
    if entity_index == local_player_index {
        println!("Skip rendering entity {entity_index}: Is the local player.");

        //global.draw_model_execute_original(this, context, state, info, bone_to_world);

        //return;
    }

    println!("{entity_index} -> {info:?}");*/

    /*let ptr = global.model_info().name_of(info.model);
    let mut i = 0;
    let i = loop {
        if ptr.add(i).read() == 0 {
            break i;
        }

        i += 1;
    };

    let slice = std::slice::from_raw_parts(ptr, i);
    let name = std::str::from_utf8_unchecked(slice);*/

    /* if name.starts_with("models/player") {
        if name.contains("shadow") {
            return;
        }

        println!("Rendering entity {entity_index} with model `{name}`.");

        // finally render
        //flat_material.set_wireframe(true);
        //global.model_render().set_material(&flat_material);
        global.draw_model_execute_original(this, context, state, info, bone_to_world);
    } else {
        //global.model_render().reset_material();
        global.draw_model_execute_original(this, context, state, info, bone_to_world);
    }*/

    global.draw_model_execute_original(this, context, state, info, bone_to_world);
}
