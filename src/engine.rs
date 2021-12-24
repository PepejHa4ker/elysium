use core::ptr::NonNull;
use sdk::Angle;
use std::borrow::Cow;
use std::ffi::CStr;

extern "C" {
    /// Raw handle to the engine.
    pub type RawEngine;
}

unsafe impl Send for RawEngine {}
unsafe impl Sync for RawEngine {}

/// The engine.
#[derive(Debug)]
#[repr(transparent)]
pub struct Engine(NonNull<RawEngine>);

impl Engine {
    pub const fn from_raw(raw: *mut RawEngine) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawEngine) -> Self {
        Self(NonNull::new_unchecked(raw))
    }

    pub const fn as_ptr(&self) -> *const RawEngine {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
    }

    pub fn clients_capacity(&self) -> i32 {
        type ClientsCapacity = unsafe extern "C" fn(this: *const RawEngine) -> i32;

        unsafe { virt::get::<ClientsCapacity>(self.virtual_table(), 20 * 8)(self.as_ptr()) }
    }

    pub fn command<'a, C>(&self, command: C)
    where
        C: Into<Cow<'a, CStr>>,
    {
        type Command = unsafe extern "C" fn(this: *const RawEngine, command: *const i8);

        unsafe {
            virt::get::<Command>(self.virtual_table(), 108 * 8)(
                self.as_ptr(),
                command.into().as_ptr(),
            )
        }
    }

    pub fn command_unrestricted<'a, C>(&self, command: C)
    where
        C: Into<Cow<'a, CStr>>,
    {
        type CommandUnrestricted = unsafe extern "C" fn(this: *const RawEngine, command: *const i8);

        unsafe {
            virt::get::<CommandUnrestricted>(self.virtual_table(), 113 * 8)(
                self.as_ptr(),
                command.into().as_ptr(),
            )
        }
    }

    pub fn in_game(&self) -> bool {
        type InGame = unsafe extern "C" fn(this: *const RawEngine) -> bool;

        unsafe { virt::get::<InGame>(self.virtual_table(), 26 * 8)(self.as_ptr()) }
    }

    pub fn is_connected(&self) -> bool {
        type IsConnected = unsafe extern "C" fn(this: *const RawEngine) -> bool;

        unsafe { virt::get::<IsConnected>(self.virtual_table(), 27 * 8)(self.as_ptr()) }
    }

    pub fn is_voice_recording(&self) -> bool {
        type IsVoiceRecording = unsafe extern "C" fn(this: *const RawEngine) -> bool;

        unsafe { virt::get::<IsVoiceRecording>(self.virtual_table(), 225 * 8)(self.as_ptr()) }
    }

    pub fn local_player(&self) -> i32 {
        type LocalPlayer = unsafe extern "C" fn(this: *const RawEngine) -> i32;

        unsafe { virt::get::<LocalPlayer>(self.virtual_table(), 12 * 8)(self.as_ptr()) }
    }

    pub fn screen_dimensions(&self) -> (i32, i32) {
        type ScreenDimensions =
            unsafe extern "C" fn(this: *const RawEngine, width: *mut i32, height: *mut i32);

        let mut size = (0, 0);

        unsafe {
            virt::get::<ScreenDimensions>(self.virtual_table(), 5 * 8)(
                self.as_ptr(),
                &mut size.0,
                &mut size.1,
            );
        }

        size
    }

    pub fn view_angle(&self) -> Angle {
        type ViewAngle = unsafe extern "C" fn(this: *const RawEngine, angle: *mut Angle);

        let mut angle = Angle::zero();

        unsafe {
            virt::get::<ViewAngle>(self.virtual_table(), 18 * 8)(self.as_ptr(), &mut angle);
        }

        angle
    }
}
