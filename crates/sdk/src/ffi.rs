use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr, slice, str};

#[inline]
pub fn cstr_cow_from_bytes<'a>(bytes: &'a [u8]) -> Option<Cow<'a, CStr>> {
    const EMPTY: &[u8] = b"\0";

    Some(match bytes.last() {
        None => Cow::Borrowed(unsafe { CStr::from_ptr(EMPTY.as_ptr().cast()) }),
        Some(&0) => Cow::Borrowed(CStr::from_bytes_with_nul(bytes).ok()?),
        Some(_) => Cow::Owned(CString::new(bytes).ok()?),
    })
}

#[inline]
pub fn cstr_cow_as_ptr(maybe_cstr: Option<&Cow<'_, CStr>>) -> *const u8 {
    let ptr = match maybe_cstr.as_ref() {
        Some(cstr) => cstr.as_ptr().cast(),
        None => ptr::null(),
    };

    ptr
}

#[inline]
pub fn osstr_to_cstr_cow<'a, S>(string: S) -> Option<Cow<'a, CStr>>
where
    S: AsRef<OsStr> + 'a,
{
    // SAFETY: silence, rustc
    let bytes: &'a [u8] = unsafe { mem::transmute(string.as_ref().as_bytes()) };
    let maybe_cstr = cstr_cow_from_bytes(bytes);

    maybe_cstr
}

#[inline]
pub unsafe fn str_from_ptr<'a>(ptr: *const u8) -> &'a str {
    let mut end = ptr;

    while end.read_unaligned() != 0 {
        end = end.add(1);
    }

    let len = end.offset_from(ptr) as usize;
    let slice = slice::from_raw_parts(ptr, len);
    let string = str::from_utf8_unchecked(slice);

    string
}
