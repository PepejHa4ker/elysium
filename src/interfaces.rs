use crate::client::Client;
use crate::console::{Console, Var};
use crate::consts::interface;
use crate::consts::var;
use crate::entity::EntityList;
use crate::globals::Globals;
use crate::libraries::Libraries;
use crate::material::Materials;
use crate::model::{ModelInfo, ModelRender};
use crate::pattern;
use crate::physics::Physics;
use core::ptr;

pub struct Interfaces {
    pub console: Console,
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

    pub cheats: Var<i32>,
    pub draw_model_stats_overlay: Var<i32>,
    pub ffa: Var<i32>,
    pub gravity: Var<f32>,
    pub infinite_ammo: Var<i32>,
    pub lost_focus_sleep: Var<i32>,
    pub model_stats_overlay: Var<i32>,
    pub panorama_blur: Var<i32>,
    pub physics_timescale: Var<f32>,
    pub post_processing: Var<i32>,
    pub ragdoll_gravity: Var<f32>,
    pub show_impacts: Var<i32>,
    
    pub shadows: Var<i32>,
    pub csm: Var<i32>,
    pub csm_shadows: Var<i32>,
    pub foot_shadows: Var<i32>,

    pub blood: Var<i32>,
    pub decals: Var<i32>,

    pub auto_help: Var<i32>,
    pub show_help: Var<i32>,
    pub html_motd: Var<i32>,
    pub freeze_cam: Var<i32>,
}

impl Interfaces {
    pub fn new(libraries: &Libraries) -> Self {
        let console = Console::new(
            libraries
                .materialsystem
                .get_interface(interface::VENGINECVAR) as _,
        )
        .unwrap();

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

        let cheats = console.var(var::CHEATS).unwrap();
        let draw_model_stats_overlay = console.var(var::DRAW_MODEL_STATS_OVERLAY).unwrap();
        let ffa = console.var(var::FFA).unwrap();
        let gravity = console.var(var::GRAVITY).unwrap();
        let infinite_ammo = console.var(var::INFINITE_AMMO).unwrap();
        let lost_focus_sleep = console.var(var::LOST_FOCUS_SLEEP).unwrap();
        let model_stats_overlay = console.var(var::MODEL_STATS_OVERLAY).unwrap();
        let panorama_blur = console.var(var::PANORAMA_BLUR).unwrap();
        let physics_timescale = console.var(var::PHYSICS_TIMESCALE).unwrap();
        let post_processing = console.var(var::POST_PROCESS).unwrap();
        let ragdoll_gravity = console.var(var::RAGDOLL_GRAVITY).unwrap();
        let show_impacts = console.var(var::SHOW_IMPACTS).unwrap();
        
        let shadows = console.var(var::SHADOWS).unwrap();
        let csm = console.var(var::CSM).unwrap();
        let csm_shadows = console.var(var::CSM_SHADOWS).unwrap();
        let foot_shadows = console.var(var::FOOT_SHADOWS).unwrap();
        
        let blood = console.var(var::BLOOD).unwrap();
        let decals = console.var(var::DECALS).unwrap();

        let show_help = console.var(var::SHOW_HELP).unwrap();
        let auto_help = console.var(var::AUTO_HELP).unwrap();
        let html_motd = console.var(var::HTML_MOTD).unwrap();
        let freeze_cam = console.var(var::FREEZE_CAM).unwrap();
        
        Self {
            shadows,
            csm,
            csm_shadows,
            foot_shadows,

            blood,
            decals,

            show_help,
            auto_help,
            html_motd,
            freeze_cam,

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

            cheats,
            draw_model_stats_overlay,
            ffa,
            gravity,
            infinite_ammo,
            lost_focus_sleep,
            model_stats_overlay,
            panorama_blur,
            physics_timescale,
            post_processing,
            ragdoll_gravity,
            show_impacts,
        }
    }
}
