use core::mem::MaybeUninit;
use frosting::ffi::vtable;
use frosting::str;
use std::ffi::OsStr;

#[repr(C)]
struct VTable {
    _unknown0: vtable::Pad<5>,
    get_screen_size: unsafe extern "C" fn(this: *const Engine, width: *mut f32, height: *mut f32),
    _unknown1: vtable::Pad<2>,
    get_player_info:
        unsafe extern "C" fn(this: *const Engine, index: i32, player_info: *mut PlayerInfo) -> bool,
    get_player_for_user: unsafe extern "C" fn(this: *const Engine, id: u64) -> i32,
    _unknown2: vtable::Pad<8>,
    get_view_angle: unsafe extern "C" fn(this: *const Engine, view_angle: *mut Vec3),
    set_view_angle: unsafe extern "C" fn(this: *const Engine, view_angle: *const Vec3),
    get_max_clients: unsafe extern "C" fn(this: *const Engine) -> i32,
    _unknown3: vtable::Pad<6>,
    is_in_game: unsafe extern "C" fn(this: *const Engine) -> bool,
    is_connected: unsafe extern "C" fn(this: *const Engine) -> bool,
    _unknown4: vtable::Pad<7>,
    set_cull_box:
        unsafe extern "C" fn(this: *const Engine, min: *const Vec3, max: *const Vec3) -> bool,
    _unknown5: vtable::Pad<3>,
    world_to_screen_matrix: unsafe extern "C" fn(this: *const Engine) -> *const Matrix3x4,
    _unknown6: vtable::Pad<6>,
    get_bsp_tree_query: unsafe extern "C" fn(this: *const Engine) -> *const (),
    _unknown7: vtable::Pad<10>,
    get_level_name: unsafe extern "C" fn(this: *const Engine) -> *const u8,
    _unknown8: vtable::Pad<25>,
    get_network_channel: unsafe extern "C" fn(this: *const Engine) -> *const NetworkChannel,
    _unknown9: vtable::Pad<25>,
}

const VTABLE_ASSERT: () = {
    assert_eq!(frosting::offset_of!(vtable.get_screen_size), 5 * 8);
    assert_eq!(frosting::offset_of!(vtable.get_player_info), 8 * 8);
    assert_eq!(frosting::offset_of!(vtable.get_player_for_user), 9 * 8);
    assert_eq!(frosting::offset_of!(vtable.get_view_angle), 18 * 8);
    assert_eq!(frosting::offset_of!(vtable.set_view_angle), 19 * 8);
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
    pub fn get_player_for_user(&self, id: Id<User>) -> Option<PlayerIndex> {
        unsafe {
            let index = (self.vtable.get_player_for_user)(self, id.get());

            Some(index)
        }
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
        unsafe { (self.vtable.get_max_client)(self) }
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
        unsafe { (self.vtable.world_to_screen_matrix)(self) }
    }

    #[inline]
    pub fn get_level_name(&self) -> &str {
        unsafe {
            let address = (self.vtable.get_level_name)(self);

            str::from_cstr(address)
        }
    }

    #[inline]
    pub fn get_network_channel(&self) -> *const NetworkChannel {
        unsafe { (self.vtable.get_network_channel)(self) }
    }

    #[inline]
    pub fn client_command<C>(&self, command: C, from_console_or_keybind: bool)
    where
        C: AsRef<OsStr>,
    {
        unsafe {
            let command = ffi::os_str(command);

            (self.vtable.client_command)(command, from_console_or_keybind);
        }
    }
}
