use crate::managed::{handle, Managed};
use elysium_math::Vec3;

/// The debug overlay interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct Debug(Managed<handle::Debug>);

impl Debug {
    pub fn new(ptr: *mut handle::Debug) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Debug) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Debug {
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

    pub fn draw_pill(
        &self,
        mins: Vec3,
        maxs: Vec3,
        diameter: f32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
        duration: f32,
    ) {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Debug,
            mins: *const Vec3,
            maxs: *const Vec3,
            diameter: *const f32,
            r: i32,
            g: i32,
            b: i32,
            a: i32,
            duration: f32,
        );

        unsafe {
            self.virtual_entry::<Fn>(23)(
                self.as_ptr(),
                &mins,
                &maxs,
                &diameter,
                r as i32,
                g as i32,
                b as i32,
                a as i32,
                duration,
            );
        }
    }
}
