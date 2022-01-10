use crate::managed::{handle, Managed};
use core::marker::PhantomData;

#[derive(Debug)]
#[repr(transparent)]
pub struct Var<T>(Managed<handle::ConsoleVar>, PhantomData<T>)
where
    T: Kind;

pub trait Sealed {}

pub trait Kind: Sealed + Sized {
    fn get(var: &Var<Self>) -> Self;
    fn set(self, var: &Var<Self>);
}

impl<T> Var<T>
where
    T: Kind,
{
    pub fn new(ptr: *mut handle::ConsoleVar) -> Option<Self> {
        Some(Self(Managed::new(ptr)?, PhantomData))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::ConsoleVar) -> Self {
        Self(Managed::new_unchecked(ptr), PhantomData)
    }

    pub fn as_ptr(&self) -> *const handle::ConsoleVar {
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

    pub fn get(&self) -> T {
        Kind::get(self)
    }

    pub fn set(&self, value: T) {
        Kind::set(value, self)
    }
}

impl Sealed for f32 {}
impl Sealed for i32 {}

impl Kind for f32 {
    fn get(var: &Var<f32>) -> Self {
        type Fn = unsafe extern "C" fn(this: *const handle::ConsoleVar) -> f32;

        unsafe { var.virtual_entry::<Fn>(15)(var.as_ptr()) }
    }

    fn set(self, var: &Var<f32>) {
        type Fn = unsafe extern "C" fn(this: *const handle::ConsoleVar, value: f32);

        unsafe { var.virtual_entry::<Fn>(18)(var.as_ptr(), self) }
    }
}

impl Kind for i32 {
    fn get(var: &Var<i32>) -> Self {
        type Fn = unsafe extern "C" fn(this: *const handle::ConsoleVar) -> i32;

        unsafe { var.virtual_entry::<Fn>(16)(var.as_ptr()) }
    }

    fn set(self, var: &Var<i32>) {
        type Fn = unsafe extern "C" fn(this: *const handle::ConsoleVar, value: i32);

        unsafe { var.virtual_entry::<Fn>(19)(var.as_ptr(), self) }
    }
}
