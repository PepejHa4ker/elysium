use super::{ffi, NetworkChannel, SteamAPIContext, SteamId};
use elysium_math::{Matrix3x4, Vec3};
use frosting::ffi::vtable;
use frosting::str;
use std::ffi::OsStr;
use std::mem::MaybeUninit;

#[repr(C)]
pub struct PlayerInfo {
    pub version: u64,
    pub steam_id: u64,
    pub name: [u8; 128],
    pub user_id: i32,
    pub guid: [u8; 33],
    pub friends_id: u32,
    pub fake_player: bool,
    pub hltv: bool,
    pub custom_files: [i32; 4],
    pub files_downloaded: u8,
}

#[repr(C)]
struct VTable {
    _unknown0: vtable::Pad<5>,
    get_screen_size: unsafe extern "C" fn(this: *const Engine, width: *mut f32, height: *mut f32),
    _unknown1: vtable::Pad<2>,
    get_player_info:
        unsafe extern "C" fn(this: *const Engine, index: i32, player_info: *mut PlayerInfo) -> bool,
    get_player_for_user_id: unsafe extern "C" fn(this: *const Engine, id: u64) -> i32,
    _unknown2: vtable::Pad<2>,
    local_player_index: unsafe extern "C" fn(this: *const Engine) -> i32,
    _unknown3: vtable::Pad<5>,
    get_view_angle: unsafe extern "C" fn(this: *const Engine, view_angle: *mut Vec3),
    set_view_angle: unsafe extern "C" fn(this: *const Engine, view_angle: *const Vec3),
    get_max_clients: unsafe extern "C" fn(this: *const Engine) -> i32,
    _unknown4: vtable::Pad<5>,
    is_in_game: unsafe extern "C" fn(this: *const Engine) -> bool,
    is_connected: unsafe extern "C" fn(this: *const Engine) -> bool,
    _unknown5: vtable::Pad<6>,
    set_cull_box:
        unsafe extern "C" fn(this: *const Engine, min: *const Vec3, max: *const Vec3) -> bool,
    _unknown6: vtable::Pad<2>,
    world_to_screen_matrix: unsafe extern "C" fn(this: *const Engine) -> *const Matrix3x4,
    _unknown7: vtable::Pad<5>,
    get_bsp_tree_query: unsafe extern "C" fn(this: *const Engine) -> *const (),
    _unknown8: vtable::Pad<9>,
    get_level_name: unsafe extern "C" fn(this: *const Engine) -> *const u8,
    _unknown9: vtable::Pad<24>,
    get_network_channel: unsafe extern "C" fn(this: *const Engine) -> *const NetworkChannel,
    _unknown10: vtable::Pad<34>,
    command: unsafe extern "C" fn(
        this: *const Engine,
        command: *const u8,
        from_console_or_keybind: bool,
    ),
    _unknown11: vtable::Pad<72>,
    get_steam_api_context: unsafe extern "C" fn(this: *const Engine) -> *const SteamAPIContext,
}

#[allow(dead_code)]
#[allow(invalid_value)]
const VTABLE_VALIDATION: () = {
    let vtable: VTable = unsafe { MaybeUninit::uninit().assume_init() };

    if frosting::offset_of!(vtable.get_screen_size) != 5 * 8 {
        panic!("invalid vtable.get_screen_size offset");
    }

    if frosting::offset_of!(vtable.get_player_info) != 8 * 8 {
        panic!("invalid vtable.get_player_info offset");
    }

    if frosting::offset_of!(vtable.get_player_for_user_id) != 9 * 8 {
        panic!("invalid vtable.get_player_for_user_id offset");
    }

    if frosting::offset_of!(vtable.local_player_index) != 12 * 8 {
        panic!("invalid vtable.local_player_index offset");
    }

    if frosting::offset_of!(vtable.get_view_angle) != 18 * 8 {
        panic!("invalid vtable.get_view_angle offset");
    }

    if frosting::offset_of!(vtable.set_view_angle) != 19 * 8 {
        panic!("invalid vtable.set_view_angle offset");
    }

    if frosting::offset_of!(vtable.get_max_clients) != 20 * 8 {
        panic!("invalid vtable.get_max_clients offset");
    }

    if frosting::offset_of!(vtable.is_in_game) != 26 * 8 {
        panic!("invalid vtable.is_in_game offset");
    }

    if frosting::offset_of!(vtable.is_connected) != 27 * 8 {
        panic!("invalid vtable.is_connected offset");
    }

    if frosting::offset_of!(vtable.set_cull_box) != 34 * 8 {
        panic!("invalid vtable.set_cull_box offset");
    }

    if frosting::offset_of!(vtable.world_to_screen_matrix) != 37 * 8 {
        panic!("invalid vtable.world_to_screen_matrix offset");
    }

    if frosting::offset_of!(vtable.get_bsp_tree_query) != 43 * 8 {
        panic!("invalid vtable.get_bsp_tree_query offset");
    }

    if frosting::offset_of!(vtable.get_level_name) != 53 * 8 {
        panic!("invalid vtable.get_level_name offset");
    }

    if frosting::offset_of!(vtable.get_network_channel) != 78 * 8 {
        panic!("invalid vtable.get_network_channel offset");
    }

    if frosting::offset_of!(vtable.command) != 113 * 8 {
        panic!("invalid vtable.command offset");
    }

    if frosting::offset_of!(vtable.get_steam_api_context) != 186 * 8 {
        panic!("invalid vtable.get_steam_api_context offset");
    }
};

#[repr(C)]
pub struct Engine {
    vtable: &'static VTable,
}

impl Engine {
    #[inline]
    pub fn get_screen_size(&self) -> (f32, f32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();

            (self.vtable.get_screen_size)(self, width.as_mut_ptr(), height.as_mut_ptr());

            (width.assume_init(), height.assume_init())
        }
    }

    #[inline]
    pub fn get_player_info(&self, index: i32) -> Option<PlayerInfo> {
        unsafe {
            let mut player_info = MaybeUninit::uninit();
            let exists = (self.vtable.get_player_info)(self, index, player_info.as_mut_ptr());

            if exists {
                Some(player_info.assume_init())
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get_player_for_user_id(&self, id: SteamId) -> Option<i32> {
        unsafe {
            let index = (self.vtable.get_player_for_user_id)(self, id.0);

            Some(index)
        }
    }

    #[inline]
    pub fn local_player_index(&self) -> i32 {
        unsafe { (self.vtable.local_player_index)(self) }
    }

    #[inline]
    pub fn get_view_angle(&self) -> Vec3 {
        unsafe {
            let mut view_angle = MaybeUninit::uninit();

            (self.vtable.get_view_angle)(self, view_angle.as_mut_ptr());

            view_angle.assume_init()
        }
    }

    #[inline]
    pub fn set_view_angle(&self, view_angle: Vec3) {
        unsafe { (self.vtable.set_view_angle)(self, &view_angle) }
    }

    #[inline]
    pub fn get_max_clients(&self) -> i32 {
        unsafe { (self.vtable.get_max_clients)(self) }
    }

    #[inline]
    pub fn is_in_game(&self) -> bool {
        unsafe { (self.vtable.is_in_game)(self) }
    }

    #[inline]
    pub fn is_connected(&self) -> bool {
        unsafe { (self.vtable.is_connected)(self) }
    }

    #[inline]
    pub fn set_cull_box(&self, min: Vec3, max: Vec3) {
        unsafe {
            (self.vtable.set_cull_box)(self, &min, &max);
        }
    }

    #[inline]
    pub fn world_to_screen_matrix(&self) -> Matrix3x4 {
        unsafe { *(self.vtable.world_to_screen_matrix)(self) }
    }

    #[inline]
    pub fn get_bsp_tree_query(&self) -> *const () {
        unsafe { (self.vtable.get_bsp_tree_query)(self) }
    }

    #[inline]
    pub fn get_level_name(&self) -> &str {
        unsafe {
            let address = (self.vtable.get_level_name)(self);

            ffi::str_from_ptr(address)
        }
    }

    #[inline]
    pub fn get_network_channel(&self) -> *const NetworkChannel {
        unsafe { (self.vtable.get_network_channel)(self) }
    }

    #[inline]
    pub fn command<C>(&self, command: C, from_console_or_keybind: bool)
    where
        C: AsRef<OsStr>,
    {
        unsafe {
            let maybe_cstr = ffi::osstr_to_cstr_cow(command);
            let ptr = ffi::cstr_cow_as_ptr(maybe_cstr.as_ref());

            (self.vtable.command)(self, ptr, from_console_or_keybind);
        }
    }
}
