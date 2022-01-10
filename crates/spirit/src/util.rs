use core::mem;

pub const fn len<T>(ptr: *const T) -> usize
where
    [(); mem::size_of::<T>()]:,
{
    if mem::size_of::<T>() == 0 {
        return 0;
    }

    let ptr = ptr as *const [u8; mem::size_of::<T>()];
    let mut len = 0;

    while unsafe {
        let ptr = ptr.add(len) as *const u8;
        let mut byte_len = 0;

        while *ptr.add(byte_len) == 0 && byte_len < mem::size_of::<T>() {
            byte_len += 1;
        }

        byte_len != mem::size_of::<T>()
    } {
        len += 1;
    }

    len
}
