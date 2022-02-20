use crate::{panic, validate};
use core::ptr;

const fn enough_for(j: usize, n: usize) {
    if j > n {
        panic::increase_size();
    }
}

const fn reduce(j: usize, n: usize) {
    if j < n {
        panic::decrease_size();
    } else if j > n {
        panic::increase_size();
    }
}

const PREFIX: [u8; 8] = *b"(?msx-u)";

pub const fn parse_pattern<const N: usize>(pattern: &'static str) -> [u8; N] {
    use validate::Kind::{Digit, Wildcard};

    if N < 9 {
        panic::increase_size();
    }

    validate::validate_pattern(pattern);

    let src = pattern.as_ptr();
    let mut i = 0;
    let len = pattern.len();
    let rem = len % 3;

    let mut pattern = [0_u8; N];
    let dst = pattern.as_mut_ptr();
    let mut j = 8;

    unsafe {
        ptr::copy_nonoverlapping(PREFIX.as_ptr(), dst, 8);
    }

    while i + 2 < len {
        let [a, b, _] = unsafe { *(src.add(i) as *const [u8; 3]) };

        match validate::kind_of_pair(a, b) {
            (Digit, Digit) => unsafe {
                enough_for(j + 4, N);

                dst.add(j + 0).write(b'\\');
                dst.add(j + 1).write(b'x');
                dst.add(j + 2).write(a | 32);
                dst.add(j + 3).write(b | 32);

                j += 4;
            },
            (Wildcard, Wildcard) => unsafe {
                enough_for(j + 1, N);

                dst.add(j).write(b'.');

                j += 1;
            },
            _ => {}
        }

        i += 3;
    }

    if rem == 2 {
        let [a, b] = unsafe { *(src.add(i) as *const [u8; 2]) };

        match validate::kind_of_pair(a, b) {
            (Digit, Digit) => unsafe {
                enough_for(j + 4, N);

                dst.add(j + 0).write(b'\\');
                dst.add(j + 1).write(b'x');
                dst.add(j + 2).write(a | 32);
                dst.add(j + 3).write(b | 32);

                j += 4;
            },
            (Wildcard, Wildcard) => unsafe {
                enough_for(j + 1, N);

                dst.add(j).write(b'.');

                j += 1;
            },
            _ => {}
        }
    }

    reduce(j, N);

    pattern
}
