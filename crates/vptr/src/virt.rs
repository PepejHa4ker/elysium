use crate::Pointer;

pub trait Virtual<T> {
    /// Obtain a pointer to this pointers virtual table.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn vtable(self) -> *const *const ();

    /// Read a virtual object.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn vget<U>(self, offset: isize) -> U;

    /// Obtain a pointer to a virtual object.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn voffset<U>(self, offset: isize) -> *const U;
}

pub trait VirtualMut<T> {
    /// Obtain a pointer to this pointers virtual table.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn vtable(self) -> *mut *mut ();

    /// Read a virtual object.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn vget<U>(self, offset: isize) -> U;

    /// Obtain a pointer to a virtual object.
    ///
    /// # Safety
    ///
    /// This pointer must point to a virtual table
    ///
    unsafe fn voffset<U>(self, offset: isize) -> *mut U;

    /// Replace a virtual object, returning the original object.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `base` must be [valid] for both reads and writes.
    ///
    /// * `base` must be properly aligned.
    ///
    /// * `base` must point to a properly initialized value that contains a virtual table.
    ///
    /// * `offset` must be a valid offset within the virtual table.
    ///
    unsafe fn vreplace<U>(self, src: U, offset: isize) -> U;

    /// Changes memory protections for a virtual object, replaces the virtual object, and returns the original object.
    ///
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// * `base` must be [valid] for both reads and writes.
    ///
    /// * `base` must be properly aligned.
    ///
    /// * `base` must point to a properly initialized value that contains a virtual table.
    ///
    /// * `offset` must be a valid offset within the virtual table.
    ///
    unsafe fn vreplace_protected<U>(self, src: U, offset: isize) -> U;
}

impl<T> const Virtual<T> for *const T {
    #[inline]
    unsafe fn vtable(self) -> *const *const () {
        *(self as *const *const *const ())
    }

    #[inline]
    unsafe fn vget<U>(self, offset: isize) -> U
    where
        U: Sized,
    {
        self.voffset::<U>(offset).read()
    }

    #[inline]
    unsafe fn voffset<U>(self, offset: isize) -> *const U {
        self.vtable().offset_bytes(offset) as *const U
    }
}

impl<T> VirtualMut<T> for *mut T {
    #[inline]
    unsafe fn vtable(self) -> *mut *mut () {
        *(self as *mut *mut *mut ())
    }

    #[inline]
    unsafe fn vget<U>(self, offset: isize) -> U
    where
        U: Sized,
    {
        self.voffset::<U>(offset).read()
    }

    #[inline]
    unsafe fn voffset<U>(self, offset: isize) -> *mut U {
        self.vtable().offset_bytes(offset) as *mut U
    }

    #[inline]
    unsafe fn vreplace<U>(self, src: U, offset: isize) -> U
    where
        U: Sized,
    {
        self.voffset::<U>(offset).replace(src)
    }

    #[inline]
    unsafe fn vreplace_protected<U>(self, src: U, offset: isize) -> U
    where
        U: Sized,
    {
        let vptr = self.voffset::<*const ()>(offset);
        let prot = crate::unprotect(vptr as *const ());
        let orig = self.vreplace(src, offset);

        crate::protect(vptr as *const (), prot);

        orig
    }
}
