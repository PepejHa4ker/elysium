use crate::panic;

pub enum Kind {
    Digit,
    Space,
    Wildcard,
}

pub const fn kind_of(byte: u8) -> Kind {
    match byte {
        b'A'..=b'F' | b'0'..=b'9' => Kind::Digit,
        b' ' => Kind::Space,
        b'?' => Kind::Wildcard,
        _ => panic::invalid_character(),
    }
}

pub const fn kind_of_pair(a: u8, b: u8) -> (Kind, Kind) {
    (kind_of(a), kind_of(b))
}

const fn validate_two(a: u8, b: u8) {
    use Kind::{Digit, Space, Wildcard};

    match kind_of_pair(a, b) {
        (Digit, Space) => panic::expected_character_in_octal(),
        (Digit, Wildcard) => panic::unexpected_character_in_octal(),
        (Wildcard, Digit) => panic::unexpected_character_in_wildcard(),
        (Wildcard, Space) => panic::expected_character_in_wildcard(),
        _ => {}
    }
}

const fn validate_three(a: u8, b: u8, c: u8) {
    validate_two(a, b);

    match kind_of(c) {
        Kind::Space => {}
        _ => panic::expected_space(),
    }
}

pub const fn validate_pattern(pattern: &'static str) {
    let data_address = pattern.as_ptr();
    let len = pattern.len();

    // dont give an empty pattern
    if len > 0 {
        let a = unsafe { *data_address };

        match kind_of(a) {
            Kind::Space => panic::unexpected_space(),
            _ => {}
        }
    }

    let rem = len % 3;
    let mut index = 0;

    // validate in groups of 3
    while index + 2 < len {
        let [a, b, c] = unsafe { *(data_address.add(index) as *const [u8; 3]) };

        validate_three(a, b, c);

        index += 3;
    }

    // check remainder
    if rem == 2 {
        let [a, b] = unsafe { *(data_address.add(index) as *const [u8; 2]) };

        validate_two(a, b);
    }

    if rem == 1 {
        let a = unsafe { *data_address.add(index) };

        use Kind::{Digit, Space, Wildcard};

        match kind_of(a) {
            Digit => panic::expected_character_in_octal(),
            Wildcard => panic::expected_character_in_wildcard(),
            Space => panic::unexpected_trailing_space(),
        }
    }

    // backtrack to check if last shit had a space
    if rem == 0 {
        let a = unsafe { *data_address.add(index) };

        match kind_of(a) {
            Kind::Space => panic::unexpected_trailing_space(),
            _ => {}
        }
    }
}
