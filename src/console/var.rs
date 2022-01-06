use core::marker::PhantomData;
use core::ptr::NonNull;

extern "C" {
    /// Raw handle to a variable.
    pub type RawVar;
}

unsafe impl Send for RawVar {}
unsafe impl Sync for RawVar {}

#[derive(Clone, Debug)]
pub struct Var<T>(NonNull<RawVar>, PhantomData<T>)
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
    /// Creates a new `EntityList` list if `raw` is non-null.
    pub const fn from_raw(raw: *mut RawVar) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw_unchecked(raw) })
        }
    }

    pub const unsafe fn from_raw_unchecked(raw: *mut RawVar) -> Self {
        Self(NonNull::new_unchecked(raw), PhantomData)
    }

    pub const fn as_ptr(&self) -> *const RawVar {
        self.0.as_ptr()
    }

    pub const fn virtual_table(&self) -> *const () {
        unsafe { *(self.as_ptr() as *const *const ()) }
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
        type Get = unsafe extern "C" fn(this: *const RawVar) -> f32;

        unsafe { virt::get::<Get>(var.virtual_table(), 15 * 8)(var.as_ptr()) }
    }

    fn set(self, var: &Var<f32>) {
        type Set = unsafe extern "C" fn(this: *const RawVar, value: f32);

        unsafe { virt::get::<Set>(var.virtual_table(), 18 * 8)(var.as_ptr(), self) }
    }
}

impl Kind for i32 {
    fn get(var: &Var<i32>) -> Self {
        type Get = unsafe extern "C" fn(this: *const RawVar) -> i32;

        unsafe { virt::get::<Get>(var.virtual_table(), 16 * 8)(var.as_ptr()) }
    }

    fn set(self, var: &Var<i32>) {
        type Set = unsafe extern "C" fn(this: *const RawVar, value: i32);

        unsafe { virt::get::<Set>(var.virtual_table(), 19 * 8)(var.as_ptr(), self) }
    }
}
