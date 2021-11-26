use crate::consts::interface;
use crate::libraries::Libraries;
use core::mem;
use vptr::{Pointer, VirtualMut};

#[derive(Debug)]
pub struct Interfaces {
    pub console: *mut (),
    pub client: *mut (),
    pub client_mode: *mut (),
    pub engine: *mut (),
    pub panel: *mut (),
    pub entities: *mut (),
    pub engine_vgui: *mut (),
    pub model: *mut (),
    pub model_info: *mut (),
    pub materialsystem: *mut (),
    pub sound: *mut (),
    pub trace: *mut (),
    pub movement: *mut (),
    pub prediction: *mut (),
    pub events: *mut (),
}

impl Interfaces {
    pub fn new(libraries: &Libraries) -> Self {
        let console = libraries
            .materialsystem
            .get_interface(interface::VENGINECVAR);

        let client = libraries.client.get_interface(interface::VCLIENT);
        let engine = libraries.engine.get_interface(interface::VENGINECLIENT);
        let panel = libraries.vgui2.get_interface(interface::VENGINEVGUI);
        let entities = libraries.client.get_interface(interface::VCLIENTENTITYLIST);
        let engine_vgui = libraries.engine.get_interface(interface::VENGINEVGUI);
        let model = libraries.engine.get_interface(interface::VENGINEMODEL);
        let model_info = libraries.engine.get_interface(interface::VMODELINFOCLIENT);
        let trace = libraries.engine.get_interface(interface::ENGINETRACECLIENT);
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
            let hud_process_input: *const () = client.vget(10 * 8);
            let get_client_mode = hud_process_input.add_bytes(11).to_offset_absolute(1, 5);
            let get_client_mode: unsafe extern "C" fn() -> *mut () =
                mem::transmute(get_client_mode);
            let client_mode = get_client_mode();

            client_mode
        };

        Self {
            console,
            client,
            client_mode,
            engine,
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
        }
    }
}
