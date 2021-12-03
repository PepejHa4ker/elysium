use sdk::Angle;
use vptr::Virtual;

#[derive(Debug)]
pub struct Engine {
    this: *const (),
}

impl Engine {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
    }

    pub fn local_player_index(&self) -> i32 {
        type Signature = unsafe extern "C" fn(this: *const ()) -> i32;

        let method: Signature = unsafe { self.as_ptr().vget(12 * 8) };

        unsafe { method(self.as_ptr()) }
    }

    pub fn view_angle(&self) -> Angle {
        type Signature = unsafe extern "C" fn(this: *const (), angle: *mut Angle);

        let method: Signature = unsafe { self.as_ptr().vget(18 * 8) };
        let mut angle = Angle::zero();

        unsafe {
            method(self.as_ptr(), &mut angle);
        }

        angle
    }

    pub fn screen_size(&self) -> (i32, i32) {
        type Signature = unsafe extern "C" fn(this: *const (), angle: *mut i32, *mut i32);

        let method: Signature = unsafe { self.as_ptr().vget(5 * 8) };
        let mut size = (0, 0);

        unsafe {
            method(self.as_ptr(), &mut size.0, &mut size.1);
        }

        size
    }

    pub fn clients_capacity(&self) -> i32 {
        type Signature = unsafe extern "C" fn(this: *const ()) -> i32;

        let method: Signature = unsafe { self.as_ptr().vget(20 * 8) };

        unsafe { method(self.as_ptr()) }
    }

    pub fn in_game(&self) -> bool {
        type Signature = unsafe extern "C" fn(this: *const ()) -> bool;

        let method: Signature = unsafe { self.as_ptr().vget(26 * 8) };

        unsafe { method(self.as_ptr()) }
    }

    pub fn is_connected(&self) -> bool {
        type Signature = unsafe extern "C" fn(this: *const ()) -> bool;

        let method: Signature = unsafe { self.as_ptr().vget(27 * 8) };

        unsafe { method(self.as_ptr()) }
    }

    pub fn is_voice_recording(&self) -> bool {
        type Signature = unsafe extern "C" fn(this: *const ()) -> bool;

        let method: Signature = unsafe { self.as_ptr().vget(225 * 8) };

        unsafe { method(self.as_ptr()) }
    }

    pub fn execute_client_command(&self, command: &str) {
        type Signature = unsafe extern "C" fn(this: *const (), command: *const u8);

        let method: Signature = unsafe { self.as_ptr().vget(108 * 8) };

        unsafe { method(self.as_ptr(), command.as_ptr()) }
    }

    pub fn execute_client_command_unrestricted(&self, command: &str) {
        type Signature = unsafe extern "C" fn(this: *const (), command: *const u8);

        let method: Signature = unsafe { self.as_ptr().vget(113 * 8) };

        unsafe { method(self.as_ptr(), command.as_ptr()) }
    }
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}
