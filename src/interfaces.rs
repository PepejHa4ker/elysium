use crate::client::Client;
use crate::console::{Console, Var};
use crate::consts::interface;
use crate::engine::Engine;
use crate::entity::EntityList;
use crate::globals::Globals;
use crate::input::Input;
use crate::libraries::Libraries;
use crate::pattern;
use crate::trace::EngineTrace;
use core::mem;
use vptr::Pointer;

#[derive(Debug)]
pub struct Interfaces {
    pub console: Console,
    pub client: Client,
    pub client_mode: *mut (),
    pub engine: Engine,
    pub globals: &'static Globals,
    pub input: &'static Input,
    pub panel: *mut (),
    pub entity_list: EntityList,
    pub engine_vgui: *mut (),
    pub model: *mut (),
    pub model_info: *mut (),
    pub materialsystem: *mut (),
    pub sound: *mut (),
    pub trace: EngineTrace,
    pub movement: *mut (),
    pub prediction: *mut (),
    pub events: *mut (),

    /// offset for animation layers
    pub animation_layers: u32,

    /// offset for animation state
    pub animation_state: u32,

    /// enable variables locked behing cheats (`sv_cheats`)
    pub cheats: Var<i32>,

    /// are teammates enenmies?
    pub ffa: Var<i32>,

    /// server gravity
    pub gravity: Var<f32>,

    /// server ammo
    pub infinite_ammo: Var<i32>,

    /// sleep when the game loses focus
    pub lost_focus_sleep: Var<i32>,

    /// used for chaming ragdolls and other world props/entities
    pub model_stats_overlay: Var<i32>,

    /// panorama ui's blur
    pub panorama_blur: Var<i32>,

    /// speed of physics entities
    pub physics_timescale: Var<f32>,

    /// control whether to appply post processing effects
    pub post_processing: Var<i32>,

    /// gravity applied to ragdolls
    pub ragdoll_gravity: Var<f32>,

    /// show bullet impacts
    pub show_impacts: Var<i32>,
}

impl Interfaces {
    pub fn new(libraries: &Libraries) -> Self {
        let console = unsafe {
            Console::from_raw(
                libraries
                    .materialsystem
                    .get_interface(interface::VENGINECVAR),
            )
        };

        let client =
            Client::from_raw(libraries.client.get_interface(interface::VCLIENT) as _).unwrap();

        let engine =
            unsafe { Engine::from_raw(libraries.engine.get_interface(interface::VENGINECLIENT)) };

        let panel = libraries.vgui2.get_interface(interface::VENGINEVGUI);

        let entity_list =
            EntityList::from_raw(libraries.client.get_interface(interface::VCLIENTENTITYLIST) as _)
                .unwrap();

        let engine_vgui = libraries.engine.get_interface(interface::VENGINEVGUI);

        let model = libraries.engine.get_interface(interface::VENGINEMODEL);

        let model_info = libraries.engine.get_interface(interface::VMODELINFOCLIENT);

        let trace = unsafe {
            EngineTrace::from_raw(libraries.engine.get_interface(interface::ENGINETRACECLIENT))
        };

        let movement = libraries.engine.get_interface(interface::GAMEMOVEMENT);

        let materialsystem = libraries
            .materialsystem
            .get_interface(interface::VMATERIALSYSTEM);

        let sound = libraries
            .engine
            .get_interface(interface::IENGINESOUNDCLIENT);

        let prediction = libraries
            .client
            .get_exact_interface(interface::VCLIENTPREDICTION);

        let events = libraries
            .client
            .get_exact_interface(interface::GAMEVENTSMANAGER);

        let client_mode = unsafe {
            let hud_process_input = client.hud_process_input_ptr();
            let get_client_mode = hud_process_input.add_bytes(11).to_offset_absolute(1, 5);
            let get_client_mode: unsafe extern "C" fn() -> *mut () =
                mem::transmute(get_client_mode);
            let client_mode = get_client_mode();

            client_mode
        };

        let globals = unsafe {
            let hud_update = client.hud_update_ptr();
            let globals =
                *(hud_update.add_bytes(13).to_offset_absolute(3, 7) as *const *const Globals);

            &*globals
        };

        let input = unsafe {
            let activate_mouse = client.activate_mouse_ptr();
            let input = **(activate_mouse.to_offset_absolute(3, 7) as *const *const *const Input);

            &*input
        };

        let patterns = pattern::Libraries::new();
        let animation_layers = unsafe {
            *(patterns
                .address_of("client_client.so", &pattern::ANIMATION_LAYERS)
                .unwrap()
                .add(35) as *const u32)
        };

        let animation_state = unsafe {
            *(patterns
                .address_of("client_client.so", &pattern::ANIMATION_STATE)
                .unwrap()
                .add(52) as *const u32)
        };

        let cheats = console.var("sv_cheats").unwrap();
        let ffa = console.var("mp_teammates_are_enemies").unwrap();
        let gravity = console.var("sv_gravity").unwrap();
        let infinite_ammo = console.var("sv_infinite_ammo").unwrap();
        let lost_focus_sleep = console.var("engine_no_focus_sleep").unwrap();
        let model_stats_overlay = console.var("r_drawmodelstatsoverlay").unwrap();
        let panorama_blur = console.var("@panorama_disable_blur").unwrap();
        let physics_timescale = console.var("cl_phys_timescale").unwrap();
        let post_processing = console.var("mat_postprocess_enable").unwrap();
        let ragdoll_gravity = console.var("cl_ragdoll_gravity").unwrap();
        let show_impacts = console.var("sv_showimpacts").unwrap();

        Self {
            console,
            client,
            client_mode,
            engine,
            globals,
            input,
            panel,
            entity_list,
            engine_vgui,
            model,
            model_info,
            materialsystem,
            sound,
            trace,
            movement,
            prediction,
            events,

            animation_layers,
            animation_state,

            cheats,
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
