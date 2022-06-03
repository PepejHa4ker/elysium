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

        let animation_state = unsafe {
            *(patterns
                .address_of("client_client.so", &pattern::ANIMATION_STATE)
                .unwrap_or(ptr::null())
                .add(52) as *const u32)
        };

        println!(
            "Searching for pattern {:?} (host_run_frame_input) in `engine_client.so'",
            pattern::HOST_RUN_FRAME_INPUT
        );

        let host_run_frame_input = unsafe {
            patterns
                .address_of("engine_client.so", &pattern::HOST_RUN_FRAME_INPUT)
                .unwrap_or(ptr::null())
        };

        println!(
            "Searching for pattern {:?} (cl_move) in `engine_client.so'",
            pattern::CL_MOVE
        );

        // refer to patterns.rs
        let cl_move = unsafe {
            let cl_move = patterns
                .address_of("engine_client.so", &pattern::CL_MOVE)
                .unwrap_or(ptr::null());

            let cl_move: elysium_state::ClMoveFn = core::mem::transmute(cl_move);
            
            elysium_state::set_cl_move(cl_move);

            cl_move
        };

        println!("host_run_frame_input = {host_run_frame_input:?}");
        println!("host_run_frame_input = {:02X?}", unsafe { host_run_frame_input.cast::<[u8; 39]>().read() });

        println!("cl_move = {cl_move:?}");
        println!("cl_move = {:02X?}", unsafe { (cl_move as *const u8).cast::<[u8; 31]>().read() });

        // e8 <relative>  call  CL_SendMove
        // 0x005929d3 - 0x00592910 = 195
        unsafe {
            let cl_move_hook = crate::hooks2::cl_move as usize as *const u8;

            println!("cl_move_hook = {:02X?}", cl_move_hook.cast::<[u8; 7]>().read());

            let call_cl_move = host_run_frame_input.byte_offset(195);

            println!("call cl_move (host_run_frame_input + 195) = {:02X?}", call_cl_move.cast::<[u8; 5]>().read());

            // obtain rip
            let rip = call_cl_move.byte_offset(5);

            // calulate relative
            let relative = cl_move_hook.byte_offset_from(rip);

            println!("cl_move_hook relative = {relative:?}");

            // remove protection
            let protection = elysium_mem::unprotect(call_cl_move);

            // replace relative
            let original = call_cl_move.byte_offset(1).cast::<i32>().as_mut().replace(relative as i32);

            println!("cl_move_hook relative (original) = {original:?}");

            // restore protection
            elysium_mem::protect(call_cl_move, protection);
            
            println!("call cl_move (host_run_frame_input + 195) (new) = {:02X?}", call_cl_move.cast::<[u8; 5]>().read());
        }

        // e8 <relative>  call  CL_SendMove
        // 0x003b5ac0 - 0x003b5740 = 896
        let cl_sendmove = unsafe {
            let base = (cl_move as *const u8).byte_offset(896);

            println!("call cl_sendmove (cl_move + 896) = {:02X?}", base.cast::<[u8; 5]>().read());

            // skip e8 in call
            let relative = base.byte_add(1).cast::<i32>().read() as isize;
            let address = elysium_mem::to_absolute(base, relative, 5);

            address
        };
        
        println!("cl_sendmove = {cl_sendmove:?}");

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
