use crate::console::Console;
//use crate::entity::EntityList;
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
    //pub globals: &'static Globals,
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


            // to be updated
            //println!("client");
            let client = &*interfaces.client.cast::<elysium_sdk::Client>();
            let entity_list = EntityList::new(interfaces.entity_list as _).unwrap();
            let model_render = ModelRender::new(interfaces.model_render as _).unwrap();
            let model_info = ModelInfo::new(interfaces.model_info as _).unwrap();
            let physics = Physics::new(interfaces.physics as _).unwrap();
            let materials = Materials::new(interfaces.material as _).unwrap();

            //println!("engine");
            let engine = interfaces.engine;
            let trace = interfaces.trace;
            //println!("client mode");
            //let client_mode = client.client_mode() as *mut ();
            //println!("globals");
            //let globals = &*(client.globals() as *const Globals);
            //println!("{globals:?}");
            //println!("input");
            //let input = client.input();

            elysium_state::set_engine(engine.cast());
            elysium_state::set_trace(trace.cast());
            //elysium_state::set_input(input.cast());

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

            Self {
                console,
                client,
                //globals,
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
