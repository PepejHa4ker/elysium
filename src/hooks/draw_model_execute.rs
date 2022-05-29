use crate::global::Global;
use crate::model::{DrawModelState, ModelRenderInfo};
use elysium_math::Matrix3x4;

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
    let global = Global::handle();

    global.draw_model_execute_original(this, context, state, info, bone_to_world);

    /*let entity_index = info.entity_index;
    let global = Global::handle();
    let flat_material = Material::new_unchecked(*elysium_state::material::flat() as _);
    let plastic_material = Material::new_unchecked(*elysium_state::material::plastic() as _);

    // obtain local player index
    let local_player_index = global
        .local_player()
        .map(|player| player.index())
        .unwrap_or(-1);

    let ptr = global.model_info().name_of(info.model);
    let mut i = 0;
    let i = if ptr.is_null() {
        0
    } else {
        loop {
            if ptr.add(i).read() == 0 {
                break i;
            }

            i += 1;
        }
    };

    let slice = std::slice::from_raw_parts(ptr, i);
    let name = std::str::from_utf8_unchecked(slice);

    // reset else you get segfault, apparently!
    global.model_render().reset_material();
    flat_material.ignore_z(false);

    global.draw_model_execute_original(this, context, state, info, bone_to_world);

    if name.starts_with("models/player") && !name.contains("shadow") {
        println!("Rendering entity {entity_index} with model `{name}`.");

        flat_material.color(COLORS[entity_index as usize % 5]);

        // ignore z on the local player gives you a segfault, too!
        if entity_index != local_player_index {
            flat_material.ignore_z(true);
        }

        plastic_material.color([0.0, 1.0, 1.0, 1.0]);
        global.model_render().set_material(&plastic_material);
        global.draw_model_execute_original(this, context, state, info, bone_to_world);

        //plastic_material.color([1.0, 0.0, 1.0, 1.0]);
        //global.model_render().set_material(&plastic_material);
        //global.draw_model_execute_original(this, context, state, info, bone_to_world);
    }*/
}

const COLORS: [[f32; 4]; 6] = [
    [1.0, 0.0, 0.0, 1.0],
    [1.0, 1.0, 0.0, 1.0],
    [0.0, 1.0, 0.0, 1.0],
    [0.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 1.0, 1.0],
    [1.0, 0.0, 1.0, 1.0],
];
