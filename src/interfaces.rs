use crate::console::Console;
use crate::entity::EntityList;
use crate::globals::Globals;
use crate::material::Materials;
use crate::model::{ModelInfo, ModelRender};
use crate::pattern;
use crate::physics::Physics;
use core::ptr;
use elysium_sdk::convar::Vars;
use elysium_sdk::Client;

pub struct Interfaces {
    pub console: &'static Console,
    pub client: &'static Client,
    pub globals: &'static Globals,
    pub entity_list: EntityList,
    pub model_render: ModelRender,
    pub model_info: ModelInfo,
    pub physics: Physics,
    pub materials: Materials,

    pub animation_layers: u32,
    pub animation_state: u32,

    pub vars: Vars,
}

impl Interfaces {
    pub fn new() -> Self {
        unsafe {
            let interfaces = crate::library::load_interfaces();

            println!("console");
            let console: &'static Console = &*interfaces.convar.cast();

            // to be updated
            println!("client");
            let client = &*interfaces.client.cast::<elysium_sdk::Client>();
            let entity_list = EntityList::new(interfaces.entity_list as _).unwrap();
            let model_render = ModelRender::new(interfaces.model_render as _).unwrap();
            let model_info = ModelInfo::new(interfaces.model_info as _).unwrap();
            let physics = Physics::new(interfaces.physics as _).unwrap();
            let materials = Materials::new(interfaces.material as _).unwrap();

            println!("engine");
            let engine = interfaces.engine;
            let trace = interfaces.trace;
            //println!("client mode");
            //let client_mode = client.client_mode() as *mut ();
            println!("globals");
            let globals = &*(client.globals() as *const Globals);
            println!("input");
            let input = client.input();

            elysium_state::set_engine(engine.cast());
            elysium_state::set_trace(trace.cast());
            elysium_state::set_input(input.cast());

            println!(
                "Searching for pattern {:?} (animation_layers) in `client_client.so'",
                pattern::ANIMATION_LAYERS
            );

            let patterns = pattern::Libraries::new();
            let animation_layers = {
                *(patterns
                    .address_of("client_client.so", &pattern::ANIMATION_LAYERS)
                    .unwrap_or(ptr::null())
                    .add(35) as *const u32)
            };

            println!(
                "Searching for pattern {:?} (animation_state) in `client_client.so'",
                pattern::ANIMATION_STATE
            );

            let animation_state = {
                *(patterns
                    .address_of("client_client.so", &pattern::ANIMATION_STATE)
                    .unwrap_or(ptr::null())
                    .add(52) as *const u32)
            };

            println!(
                "Searching for pattern {:?} (host_run_frame_input) in `engine_client.so'",
                pattern::HOST_RUN_FRAME_INPUT
            );

            let host_run_frame_input = {
                patterns
                    .address_of("engine_client.so", &pattern::HOST_RUN_FRAME_INPUT)
                    .unwrap_or(ptr::null())
            };

            println!(
                "Searching for pattern {:?} (cl_move) in `engine_client.so'",
                pattern::CL_MOVE
            );

            // refer to patterns.rs
            let cl_move = {
                let cl_move = patterns
                    .address_of("engine_client.so", &pattern::CL_MOVE)
                    .unwrap_or(ptr::null());

                let cl_move: elysium_state::ClMoveFn = core::mem::transmute(cl_move);

                elysium_state::set_cl_move(cl_move);

                cl_move
            };

            println!(
                "Searching for pattern {:?} (write_user_command) in `client_client.so'",
                pattern::WRITE_USER_COMMAND
            );

            // refer to patterns.rs
            let write_user_command = {
                let write_user_command = patterns
                    .address_of("client_client.so", &pattern::WRITE_USER_COMMAND)
                    .unwrap_or(ptr::null());

                let write_user_command: elysium_state::WriteUserCommandFn =
                    core::mem::transmute(write_user_command);

                elysium_state::set_write_user_command(write_user_command);

                write_user_command
            };

            println!("host_run_frame_input = {host_run_frame_input:?}");
            println!("host_run_frame_input = {:02X?}", {
                host_run_frame_input.cast::<[u8; 39]>().read()
            });

            println!("cl_move = {cl_move:?}");
            println!("cl_move = {:02X?}", {
                (cl_move as *const u8).cast::<[u8; 31]>().read()
            });

            println!("write_user_comand = {write_user_command:?}");
            println!("write_user_command = {:02X?}", {
                (cl_move as *const u8).cast::<[u8; 20]>().read()
            });

            // e8 <relative>  call  CL_SendMove
            // 0x005929d3 - 0x00592910 = 195
            {
                let cl_move_hook = crate::hooks2::cl_move as usize as *const u8;

                println!(
                    "cl_move_hook = {:02X?}",
                    cl_move_hook.cast::<[u8; 7]>().read()
                );

                let call_cl_move = host_run_frame_input.byte_offset(195);

                println!(
                    "call cl_move (host_run_frame_input + 195) = {:02X?}",
                    call_cl_move.cast::<[u8; 5]>().read()
                );

                // obtain rip
                let rip = call_cl_move.byte_offset(5);

                // calulate relative
                let relative = cl_move_hook.byte_offset_from(rip);

                println!("cl_move_hook relative = {relative:?}");

                // remove protection
                let protection = elysium_mem::unprotect(call_cl_move);

                // replace relative
                let original = call_cl_move
                    .byte_offset(1)
                    .cast::<i32>()
                    .as_mut()
                    .replace(relative as i32);

                println!("cl_move_hook relative (original) = {original:?}");

                // restore protection
                elysium_mem::protect(call_cl_move, protection);

                println!(
                    "call cl_move (host_run_frame_input + 195) (new) = {:02X?}",
                    call_cl_move.cast::<[u8; 5]>().read()
                );
            }

            // e8 <relative>  call  CL_SendMove
            // 0x003b5ac0 - 0x003b5740 = 896
            let cl_sendmove = {
                let base = (cl_move as *const u8).byte_offset(896);

                println!(
                    "call cl_sendmove (cl_move + 896) = {:02X?}",
                    base.cast::<[u8; 5]>().read()
                );

                // skip e8 in call
                let relative = base.byte_add(1).cast::<i32>().read() as isize;
                let address = elysium_mem::to_absolute(base, relative, 5);

                address
            };

            println!("cl_sendmove = {cl_sendmove:?}");

            let vars = Vars::from_loader(|name| {
                let address = console.var(name);

                println!("convar {:?} -> {:?}", name, address);

                address
            });

            Self {
                console,
                client,
                globals,
                entity_list,
                model_render,
                model_info,
                physics,
                materials,

                animation_layers,
                animation_state,

                vars,
            }
        }
    }
}
