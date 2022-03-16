use crate::managed::{handle, Managed};
use core::mem;

pub use class::Class;
pub use classes::Classes;
pub use property::Property;
pub use table::Table;

mod class;
mod classes;
mod property;
mod table;

/// The client interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct Client(Managed<handle::Client>);

impl Client {
    pub fn new(ptr: *mut handle::Client) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Client) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Client {
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

    /// Returns an iterator over classes.
    pub fn classes(&self) -> Classes {
        type Fn = unsafe extern "C" fn(this: *const handle::Client) -> *mut Class;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(8)(self.as_ptr());

            Classes::new(ptr)
        }
    }

    /// Returns a pointer to activate_mouse.
    pub fn activate_mouse_ptr(&self) -> *const () {
        unsafe { *(self.virtual_offset(16) as *const *const ()) }
    }

    /// Returns a pointer to client_mode.
    pub fn client_mode_ptr(&self) -> *const () {
        type Fn = unsafe extern "C" fn() -> *const ();

        unsafe {
            // Locate the signed offset address.
            let signed_offset_address = (self.hud_process_input_ptr() as *const u8).add(11);

            // Determine where the signed offset address points to!
            let get_client_mode =
                providence_util::to_absolute_with_offset(signed_offset_address as *const (), 1, 5);

            // Honestly kind of sad you have to do this.
            let get_client_mode: Fn = mem::transmute(get_client_mode);

            get_client_mode()
        }
    }

    /// Returns a pointer to create_move.
    pub fn create_move_ptr(&self) -> *const () {
        unsafe { providence_util::virtual_offset(self.client_mode_ptr(), 25) }
    }

    /// Returns a pointer to frame_stage_notify.
    pub fn frame_stage_notify_ptr(&self) -> *const () {
        unsafe { self.virtual_offset(37) }
    }

    /// Returns a pointer to globals.
    pub fn globals_ptr(&self) -> *const () {
        unsafe {
            // Locate the signed offset address.
            let signed_offset_address = (self.hud_update_ptr() as *const u8).add(13);

            // Determine where the signed offset address points to!
            let globals =
                providence_util::to_absolute_with_offset(signed_offset_address as *const (), 3, 7);

            // Dereference the pointer pointing to the pointer to globals.
            *(globals as *const *const ())
        }
    }

    /// Returns a pointer to hud_process_input.
    pub fn hud_process_input_ptr(&self) -> *const () {
        unsafe { *(self.virtual_offset(10) as *const *const ()) }
    }

    /// Returns a pointer to hud_update.
    pub fn hud_update_ptr(&self) -> *const () {
        unsafe { *(self.virtual_offset(11) as *const *const ()) }
    }

    /// Returns a pointer to input.
    pub fn input_ptr(&self) -> *const () {
        unsafe {
            // Locate the signed offset address.
            let signed_offset_address = (self.activate_mouse_ptr() as *const u8).add(3);

            // Determine where the signed offset address points to!
            let input = providence_util::to_absolute(signed_offset_address as *const ());

            // Dereference the pointer pointing to the pointer to globals.
            **(input as *const *const *const ())
        }
    }
}
