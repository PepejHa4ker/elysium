use iced_native::keyboard::{Event, KeyCode, Modifiers};
use sdl2_sys::{SDL_KeyboardEvent, SDL_Scancode};

/// Map a `SDL_Scancode` to a `KeyCode`.
#[inline]
pub const fn map_key_code(key: SDL_Scancode) -> Option<KeyCode> {
    let key = match key {
        SDL_Scancode::SDL_SCANCODE_0 => KeyCode::Key0,
        SDL_Scancode::SDL_SCANCODE_1 => KeyCode::Key1,
        SDL_Scancode::SDL_SCANCODE_2 => KeyCode::Key2,
        SDL_Scancode::SDL_SCANCODE_3 => KeyCode::Key3,
        SDL_Scancode::SDL_SCANCODE_4 => KeyCode::Key4,
        SDL_Scancode::SDL_SCANCODE_5 => KeyCode::Key5,
        SDL_Scancode::SDL_SCANCODE_6 => KeyCode::Key6,
        SDL_Scancode::SDL_SCANCODE_7 => KeyCode::Key7,
        SDL_Scancode::SDL_SCANCODE_8 => KeyCode::Key8,
        SDL_Scancode::SDL_SCANCODE_9 => KeyCode::Key9,

        SDL_Scancode::SDL_SCANCODE_A => KeyCode::A,
        SDL_Scancode::SDL_SCANCODE_B => KeyCode::B,
        SDL_Scancode::SDL_SCANCODE_C => KeyCode::C,
        SDL_Scancode::SDL_SCANCODE_D => KeyCode::D,
        SDL_Scancode::SDL_SCANCODE_E => KeyCode::E,
        SDL_Scancode::SDL_SCANCODE_F => KeyCode::F,
        SDL_Scancode::SDL_SCANCODE_G => KeyCode::G,
        SDL_Scancode::SDL_SCANCODE_H => KeyCode::H,
        SDL_Scancode::SDL_SCANCODE_I => KeyCode::I,
        SDL_Scancode::SDL_SCANCODE_J => KeyCode::J,
        SDL_Scancode::SDL_SCANCODE_K => KeyCode::K,
        SDL_Scancode::SDL_SCANCODE_L => KeyCode::L,
        SDL_Scancode::SDL_SCANCODE_M => KeyCode::M,
        SDL_Scancode::SDL_SCANCODE_N => KeyCode::N,
        SDL_Scancode::SDL_SCANCODE_O => KeyCode::O,
        SDL_Scancode::SDL_SCANCODE_P => KeyCode::P,
        SDL_Scancode::SDL_SCANCODE_Q => KeyCode::Q,
        SDL_Scancode::SDL_SCANCODE_R => KeyCode::R,
        SDL_Scancode::SDL_SCANCODE_S => KeyCode::S,
        SDL_Scancode::SDL_SCANCODE_T => KeyCode::T,
        SDL_Scancode::SDL_SCANCODE_U => KeyCode::U,
        SDL_Scancode::SDL_SCANCODE_W => KeyCode::W,
        SDL_Scancode::SDL_SCANCODE_V => KeyCode::V,
        SDL_Scancode::SDL_SCANCODE_X => KeyCode::X,
        SDL_Scancode::SDL_SCANCODE_Y => KeyCode::Y,
        SDL_Scancode::SDL_SCANCODE_Z => KeyCode::Z,

        SDL_Scancode::SDL_SCANCODE_BACKSPACE => KeyCode::Backspace,
        SDL_Scancode::SDL_SCANCODE_INSERT => KeyCode::Insert,
        SDL_Scancode::SDL_SCANCODE_SPACE => KeyCode::Space,
        _ => return None,
    };

    Some(key)
}

/// Map a `KeyCode` to a `char`.
#[inline]
pub const fn map_character(key: KeyCode) -> Option<char> {
    let key = match key {
        KeyCode::Key0 => '0',
        KeyCode::Key1 => '1',
        KeyCode::Key2 => '2',
        KeyCode::Key3 => '3',
        KeyCode::Key4 => '4',
        KeyCode::Key5 => '5',
        KeyCode::Key6 => '6',
        KeyCode::Key7 => '7',
        KeyCode::Key8 => '8',
        KeyCode::Key9 => '9',

        KeyCode::A => 'a',
        KeyCode::B => 'b',
        KeyCode::C => 'c',
        KeyCode::D => 'd',
        KeyCode::E => 'e',
        KeyCode::F => 'f',
        KeyCode::G => 'g',
        KeyCode::H => 'h',
        KeyCode::I => 'i',
        KeyCode::J => 'j',
        KeyCode::K => 'k',
        KeyCode::L => 'l',
        KeyCode::M => 'm',
        KeyCode::N => 'n',
        KeyCode::O => 'o',
        KeyCode::P => 'p',
        KeyCode::Q => 'q',
        KeyCode::R => 'r',
        KeyCode::S => 's',
        KeyCode::T => 't',
        KeyCode::U => 'u',
        KeyCode::V => 'v',
        KeyCode::W => 'w',
        KeyCode::X => 'x',
        KeyCode::Y => 'y',
        KeyCode::Z => 'z',

        KeyCode::Space => ' ',

        _ => return None,
    };

    Some(key)
}

/// Map an `SDL_KeyboardEvent` to an `Event` when pressed.
#[inline]
pub fn map_key_pressed<F>(event: SDL_KeyboardEvent, mut f: F) -> Option<()>
where
    F: FnMut(Event),
{
    let key_code = map_key_code(event.keysym.scancode)?;

    f(Event::KeyPressed {
        key_code,
        modifiers: Modifiers::empty(),
    });

    let character = map_character(key_code)?;

    f(Event::CharacterReceived(character));

    Some(())
}

/// Map an `SDL_KeyboardEvent` to an `Event` when released.
#[inline]
pub fn map_key_released<F>(event: SDL_KeyboardEvent, mut f: F) -> Option<()>
where
    F: FnMut(Event),
{
    let key_code = map_key_code(event.keysym.scancode)?;

    f(Event::KeyReleased {
        key_code,
        modifiers: Modifiers::empty(),
    });

    Some(())
}
