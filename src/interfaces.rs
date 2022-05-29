use crate::client::Client;
use crate::console::Console;
use crate::consts::interface;
use crate::entity::EntityList;
use crate::globals::Globals;
use crate::libraries::Libraries;
use crate::material::Materials;
use crate::model::{ModelInfo, ModelRender};
use crate::pattern;
use crate::physics::Physics;
use core::ptr;
use elysium_sdk::convar::Vars;

pub struct Interfaces {
    pub console: &'static Console,
    pub client: Client,
    pub client_mode: *mut (),
    pub globals: &'static Globals,
    pub panel: *mut (),
    pub entity_list: EntityList,
    pub engine_vgui: *mut (),
    pub model_render: ModelRender,
    pub model_info: ModelInfo,
    pub physics: Physics,
    pub materials: Materials,
    pub sound: *mut (),
    pub movement: *mut (),
    pub prediction: *mut (),
    pub events: *mut (),

    pub animation_layers: u32,
    pub animation_state: u32,

    pub vars: Vars,
}

impl Interfaces {
    pub fn new(libraries: &Libraries) -> Self {
        let console: &'static Console = unsafe {
            &*libraries
                .materialsystem
                .get_interface(interface::VENGINECVAR)
                .cast()
        };

        let client = Client::new(libraries.client.get_interface(interface::VCLIENT) as _).unwrap();

        unsafe {
            let engine = libraries.engine.get_interface(interface::VENGINECLIENT);
            let trace = libraries.engine.get_interface(interface::ENGINETRACECLIENT);

            elysium_state::set_engine(engine.cast());
            elysium_state::set_trace(trace.cast());
        }

        let panel = libraries.vgui2.get_interface(interface::VENGINEVGUI);

        let entity_list =
            EntityList::new(libraries.client.get_interface(interface::VCLIENTENTITYLIST) as _)
                .unwrap();

        let engine_vgui = libraries.engine.get_interface(interface::VENGINEVGUI);

        let model_render =
            ModelRender::new(libraries.engine.get_interface(interface::VENGINEMODEL) as _).unwrap();

        let model_info =
            ModelInfo::new(libraries.engine.get_interface(interface::VMODELINFOCLIENT) as _)
                .unwrap();

        let physics = Physics::new(
            libraries
                .vphysics
                .get_interface(interface::VPHYSICSSURFACEPROPS) as _,
        )
        .unwrap();

        let movement = libraries.engine.get_interface(interface::GAMEMOVEMENT);

        let materials = Materials::new(
            libraries
                .materialsystem
                .get_interface(interface::VMATERIALSYSTEM) as _,
        )
        .unwrap();

        let sound = libraries
            .engine
            .get_interface(interface::IENGINESOUNDCLIENT);

        let prediction = libraries
            .client
            .get_exact_interface(interface::VCLIENTPREDICTION);

        let events = libraries
            .client
            .get_exact_interface(interface::GAMEVENTSMANAGER);

        let client_mode = client.client_mode_ptr() as *mut ();
        let globals = unsafe { &*(client.globals_ptr() as *const Globals) };

        unsafe {
            let input = client.input_ptr();

            elysium_state::set_input(input.cast());
        }

        println!(
            "Searching for pattern {:?} (animation_layers) in `client_client.so'",
            pattern::ANIMATION_LAYERS
        );

        println!("Regex: {:?}", pattern::ANIMATION_LAYERS.regex());

        let patterns = pattern::Libraries::new();
        let animation_layers = unsafe {
            *(patterns
                .address_of("client_client.so", &pattern::ANIMATION_LAYERS)
                .unwrap_or(ptr::null())
                .add(35) as *const u32)
        };

        println!(
            "Searching for pattern {:?} (animation_state) in `client_client.so'",
            pattern::ANIMATION_STATE
        );

        println!("Regex: {:?}", pattern::ANIMATION_STATE.regex());

        let animation_state = unsafe {
            *(patterns
                .address_of("client_client.so", &pattern::ANIMATION_STATE)
                .unwrap_or(ptr::null())
                .add(52) as *const u32)
        };

        let vars = unsafe {
            Vars::from_loader(|name| {
                let address = console.var(name);

                println!("convar {:?} -> {:?}", name, address);

                address
            })
        };

        Self {
            console,
            client,
            client_mode,
            globals,
            panel,
            entity_list,
            engine_vgui,
            model_render,
            model_info,
            physics,
            materials,
            sound,
            movement,
            prediction,
            events,

            animation_layers,
            animation_state,

            vars,
        }
    }
}
