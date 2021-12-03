use vptr::Virtual;

pub use self::button::Button;
pub use self::joystick::Joystick;
pub use self::mouse::Mouse;
pub use self::state::State;

mod button;
mod joystick;
mod mouse;
mod state;

#[derive(Debug)]
pub struct Input {
    this: *const (),
}

impl Input {
    pub fn as_ptr(&self) -> *const () {
        self as *const Self as *const ()
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        type Signature = unsafe extern "C" fn(this: *const (), button: Button) -> bool;

        let method: Signature = unsafe { self.as_ptr().vget(15 * 8) };

        unsafe { method(self.as_ptr(), button) }
    }
}
