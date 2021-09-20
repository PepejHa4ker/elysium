mod button;
mod joystick;
mod mouse;
mod state;

pub use self::button::Button;
pub use self::joystick::Joystick;
pub use self::mouse::Mouse;
pub use self::state::State;

#[derive(Debug)]
#[repr(C)]
pub struct Input(*const ());

impl Input {
    pub const fn as_ptr(&self) -> *const () {
        self as *const Self as _
    }

    pub const fn vtable(&self) -> *const *const *const () {
        self.as_ptr() as _
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        type IsButtonDown = unsafe extern "C" fn(this: *const (), button: Button) -> bool;

        let is_button_down: IsButtonDown =
            unsafe { std::mem::transmute(*(*self.vtable()).offset(15)) };

        unsafe { is_button_down(self.as_ptr(), button) }
    }
}
