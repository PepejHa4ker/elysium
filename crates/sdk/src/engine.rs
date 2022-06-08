use super::{ffi, vtable_export, vtable_validate, NetworkChannel, SteamAPIContext, SteamId};
use elysium_math::{Matrix3x4, Vec3};
use frosting::ffi::vtable;
use frosting::str;
use std::ffi::OsStr;
use std::mem::MaybeUninit;

/// player information
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
    get_player_for_user_id: unsafe extern "C" fn(this: *const Engine, user_id: SteamId) -> i32,
    _unknown2: vtable::Pad<2>,
    local_player_index: unsafe extern "C" fn(this: *const Engine) -> i32,
    _unknown3: vtable::Pad<5>,
    view_angle: unsafe extern "C" fn(this: *const Engine, angle: *mut Vec3),
    set_view_angle: unsafe extern "C" fn(this: *const Engine, angle: *const Vec3),
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

vtable_validate! {
    get_screen_size => 5,
    get_player_info => 8,
    get_player_for_user_id => 9,
    local_player_index => 12,
    view_angle => 18,
    set_view_angle => 19,
    get_max_clients => 20,
    is_in_game => 26,
    is_connected => 27,
    set_cull_box => 34,
    world_to_screen_matrix => 37,
    get_bsp_tree_query => 43,
    get_level_name => 53,
    get_network_channel => 78,
    command => 113,
    get_steam_api_context => 186,
}

/// engine interface
#[repr(C)]
pub struct Engine {
    vtable: &'static VTable,
}

impl Engine {
    vtable_export! {
        /// returns the maximum amount of clients
        get_max_clients() -> i32,

        /// if in game
        is_in_game() -> bool,

        /// if connected
        is_connected() -> bool,

        /// returns the bsp tree
        get_bsp_tree_query() -> *const (),

        /// returns the network channel
        get_network_channel() -> *const NetworkChannel,
    }

    /// returns the local player's index
    #[inline]
    pub fn local_player_index(&self) -> usize {
        unsafe { (self.vtable.local_player_index)(self) as usize }
    }

    /// returns the screen size
    #[inline]
    pub fn get_screen_size(&self) -> (f32, f32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();

            (self.vtable.get_screen_size)(self, width.as_mut_ptr(), height.as_mut_ptr());

            (width.assume_init(), height.assume_init())
        }
    }

    /// get player info for the player at `index`
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

    /// get player index by `user_id`
    #[inline]
    pub fn get_player_for_user_id(&self, user_id: SteamId) -> Option<i32> {
        unsafe {
            let index = (self.vtable.get_player_for_user_id)(self, user_id);

            Some(index)
        }
    }

    /// get the view angle
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            let mut view_angle = MaybeUninit::uninit();

            (self.vtable.view_angle)(self, view_angle.as_mut_ptr());

            view_angle.assume_init()
        }
    }

    /// set the view angle
    #[inline]
    pub fn set_view_angle(&self, angle: Vec3) {
        unsafe { (self.vtable.set_view_angle)(self, &angle) }
    }

    /// set the cull box
    #[inline]
    pub fn set_cull_box(&self, min: Vec3, max: Vec3) -> bool {
        unsafe { (self.vtable.set_cull_box)(self, &min, &max) }
    }

    /// returns the world to screen matrix
    #[inline]
    pub fn world_to_screen_matrix(&self) -> Matrix3x4 {
        unsafe { *(self.vtable.world_to_screen_matrix)(self) }
    }

    /// returns the current level name
    #[inline]
    pub fn get_level_name(&self) -> &str {
        unsafe {
            let address = (self.vtable.get_level_name)(self);

            ffi::str_from_ptr(address)
        }
    }

    /// executes a command
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
