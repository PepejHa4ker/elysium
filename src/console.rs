use std::ffi::CString;
use std::marker::PhantomData;
use vptr::Virtual;

#[derive(Clone, Debug)]
pub struct Var<T>
where
    T: Kind,
{
    this: *const (),
    _boo: PhantomData<T>,
}

impl<T> Var<T>
where
    T: Kind,
{
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self {
            this: ptr,
            _boo: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
    }

    pub fn get(&self) -> T {
        Kind::get(self)
    }

    pub fn set(&self, value: T) {
        Kind::set(value, self)
    }
}

pub trait Kind: Sized {
    fn get(var: &Var<Self>) -> Self;
    fn set(self, var: &Var<Self>);
}

impl Kind for f32 {
    fn get(var: &Var<f32>) -> Self {
        type Signature = unsafe extern "C" fn(this: *const ()) -> f32;

        let method: Signature = unsafe { var.as_ptr().vget(15 * 8) };

        unsafe { method(var.as_ptr()) }
    }

    fn set(self, var: &Var<f32>) {
        type Signature = unsafe extern "C" fn(this: *const (), value: f32);

        let method: Signature = unsafe { var.as_ptr().vget(18 * 8) };

        unsafe { method(var.as_ptr(), self) }
    }
}

impl Kind for i32 {
    fn get(var: &Var<i32>) -> Self {
        type Signature = unsafe extern "C" fn(this: *const ()) -> i32;

        let method: Signature = unsafe { var.as_ptr().vget(16 * 8) };

        unsafe { method(var.as_ptr()) }
    }

    fn set(self, var: &Var<i32>) {
        type Signature = unsafe extern "C" fn(this: *const (), value: i32);

        let method: Signature = unsafe { var.as_ptr().vget(19 * 8) };

        unsafe { method(var.as_ptr(), self) }
    }
}

#[derive(Clone, Debug)]
pub struct Console {
    this: *const (),
}

impl Console {
    pub unsafe fn from_raw(ptr: *const ()) -> Self {
        Self { this: ptr }
    }

    pub fn as_ptr(&self) -> *const () {
        self.this
    }

    pub fn as_mut_ptr(&self) -> *mut () {
        self.this as *mut ()
    }

    pub fn var<T>(&self, var: impl Into<Vec<u8>>) -> Option<Var<T>>
    where
        T: Kind,
    {
        type Signature = unsafe extern "C" fn(this: *const (), var: *const i8) -> *const ();

        let var = CString::new(var).unwrap();
        let method: Signature = unsafe { self.as_ptr().vget(15 * 8) };

        unsafe {
            let ptr = method(self.as_ptr(), var.as_ptr());

            println!("{:?} {:?}", &var, &ptr);

            if ptr.is_null() {
                None
            } else {
                Some(Var::from_raw(ptr))
            }
        }
    }

    pub fn write(&self, buf: impl Into<Vec<u8>>) {
        type Signature =
            unsafe extern "C" fn(this: *const (), format: *const i8, text: *const i8) -> bool;

        let text = CString::new(buf).unwrap();
        let method: Signature = unsafe { self.as_ptr().vget(27 * 8) };

        unsafe {
            method(self.as_ptr(), "%s\0".as_ptr().cast(), text.as_ptr());
        }
    }
}

unsafe impl Send for Console {}
unsafe impl Sync for Console {}
