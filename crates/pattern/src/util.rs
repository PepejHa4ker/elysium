use regex::bytes::Regex;

pub const unsafe fn change_lifetime<'a, 'b, T>(a: &'a T) -> &'b T
where
    T: ?Sized,
{
    &*(a as *const T)
}

pub fn new_regex(pattern: &'static str) -> Regex {
    unsafe { Regex::new(pattern).unwrap_unchecked() }
}
