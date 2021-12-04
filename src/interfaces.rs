use crate::client::Client;
use crate::console::{Console, Var};
use crate::consts::interface;
use crate::engine::Engine;
use crate::entities::Entities;
use crate::globals::Globals;
use crate::libraries::Libraries;
use crate::trace::EngineTrace;
use core::mem;
use vptr::{Pointer, VirtualMut};

#[derive(Debug)]
pub struct Interfaces {
    pub console: Console,
    pub client: Client,
    pub client_mode: *mut (),
    pub engine: Engine,
    pub globals: &'static Globals,
    pub panel: *mut (),
    pub entities: Entities,
    pub engine_vgui: *mut (),
    pub model: *mut (),
    pub model_info: *mut (),
    pub materialsystem: *mut (),
    pub sound: *mut (),
    pub trace: EngineTrace,
    pub movement: *mut (),
    pub prediction: *mut (),
    pub events: *mut (),

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
            unsafe { Client::from_raw(libraries.client.get_interface(interface::VCLIENT)) };

        let engine =
            unsafe { Engine::from_raw(libraries.engine.get_interface(interface::VENGINECLIENT)) };

        let panel = libraries.vgui2.get_interface(interface::VENGINEVGUI);

        let entities = unsafe {
            Entities::from_raw(libraries.client.get_interface(interface::VCLIENTENTITYLIST))
        };

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
            let hud_process_input: *const () = client.as_mut_ptr().vget(10 * 8);
            let get_client_mode = hud_process_input.add_bytes(11).to_offset_absolute(1, 5);
            let get_client_mode: unsafe extern "C" fn() -> *mut () =
                mem::transmute(get_client_mode);
            let client_mode = get_client_mode();

            client_mode
        };

        let globals = unsafe {
            let hud_update: *const () = client.as_mut_ptr().vget(11 * 8);
            let globals =
                *(hud_update.add_bytes(13).to_offset_absolute(3, 7) as *const *const Globals);

            &*globals
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
            panel,
            entities,
            engine_vgui,
            model,
            model_info,
            materialsystem,
            sound,
            trace,
            movement,
            prediction,
            events,

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
