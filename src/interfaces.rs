use crate::consts::interface;
use crate::libraries::Libraries;
use core::mem;
use vmt::PointerExt;

#[derive(Debug)]
pub struct Interfaces {
    pub console: *mut usize,
    pub client: *mut usize,
    pub client_mode: *mut usize,
    pub engine: *mut usize,
    pub panel: *mut usize,
    pub entities: *mut usize,
    pub engine_vgui: *mut usize,
    pub model: *mut usize,
    pub model_info: *mut usize,
    pub materialsystem: *mut usize,
    pub sound: *mut usize,
    pub trace: *mut usize,
    pub movement: *mut usize,
    pub prediction: *mut usize,
    pub events: *mut usize,
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
            let hud_process_input = vmt::get(client, 10);
            let get_client_mode = vmt::get_absolute_address(hud_process_input.add_bytes(11), 1, 5);
            let get_client_mode: unsafe extern "C" fn() -> *mut usize =
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
