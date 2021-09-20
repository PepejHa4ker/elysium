use std::mem;

pub use self::button::Button;
pub use self::joystick::Joystick;
pub use self::mouse::Mouse;
pub use self::state::State;

mod button;
mod joystick;
mod mouse;
mod state;

#[derive(Debug)]
#[repr(C)]
pub struct Input {
    this: *const usize,
}

impl Input {
    fn this(&self) -> *const usize {
        self as *const Self as *const usize
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        type IsButtonDown = unsafe extern "C" fn(this: *const usize, button: Button) -> bool;

        let method = unsafe { vmt::get(self.this(), 15) };
        tracing::debug!("method {:?}", method);
        let is_button_down: IsButtonDown = unsafe { mem::transmute(method) };

        unsafe { is_button_down(self.this(), button) }
    }
}
