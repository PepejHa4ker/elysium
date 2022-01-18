use crate::managed::{handle, Managed};
use sdk::Vec3;
use std::borrow::Cow;
use std::ffi::CStr;

/// The engine interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct Engine(Managed<handle::Engine>);

impl Engine {
    pub fn new(ptr: *mut handle::Engine) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Engine) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Engine {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub fn clients_capacity(&self) -> i32 {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine) -> i32;

        unsafe { self.virtual_entry::<Fn>(20)(self.as_ptr()) }
    }

    pub fn command<'a, C>(&self, command: C)
    where
        C: Into<Cow<'a, CStr>>,
    {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine, command: *const i8);

        unsafe { self.virtual_entry::<Fn>(108)(self.as_ptr(), command.into().as_ptr()) }
    }

    pub fn command_unrestricted<'a, C>(&self, command: C)
    where
        C: Into<Cow<'a, CStr>>,
    {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine, command: *const i8);

        unsafe { self.virtual_entry::<Fn>(113)(self.as_ptr(), command.into().as_ptr()) }
    }

    pub fn in_game(&self) -> bool {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine) -> bool;

        unsafe { self.virtual_entry::<Fn>(26)(self.as_ptr()) }
    }

    pub fn is_connected(&self) -> bool {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine) -> bool;

        unsafe { self.virtual_entry::<Fn>(27)(self.as_ptr()) }
    }

    pub fn is_voice_recording(&self) -> bool {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine) -> bool;

        unsafe { self.virtual_entry::<Fn>(225)(self.as_ptr()) }
    }

    pub fn local_player(&self) -> i32 {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine) -> i32;

        unsafe { self.virtual_entry::<Fn>(12)(self.as_ptr()) }
    }

    pub fn screen_dimensions(&self) -> (i32, i32) {
        type Fn =
            unsafe extern "C" fn(this: *const handle::Engine, width: *mut i32, height: *mut i32);

        let mut size = (0, 0);

        unsafe {
            self.virtual_entry::<Fn>(5)(self.as_ptr(), &mut size.0, &mut size.1);
        }

        size
    }

    pub fn view_angle(&self) -> Vec3 {
        type Fn = unsafe extern "C" fn(this: *const handle::Engine, angle: *mut Vec3);

        let mut angle = Vec3::zero();

        unsafe {
            self.virtual_entry::<Fn>(18)(self.as_ptr(), &mut angle);
        }

        angle
    }
}
