use iced_native::mouse::{Button, Event};
use iced_native::Point;
use sdl2_sys::{SDL_MouseButtonEvent, SDL_MouseMotionEvent};

/// Maps a button index to a `Button`.
#[inline]
pub const fn map_button(button: u8) -> Button {
    match button {
        1 => Button::Left,
        2 => Button::Middle,
        3 => Button::Right,
        other => Button::Other(other),
    }
}

/// Maps a `SDL_MouseButtonEvent` to a `Event` when pressed.
#[inline]
pub fn map_button_pressed<F>(event: SDL_MouseButtonEvent, mut f: F) -> Option<()>
where
    F: FnMut(Event),
{
    let button = map_button(event.button);

    f(Event::ButtonPressed(button));

    Some(())
}

/// Maps a `SDL_MouseButtonEvent` to a `Event` when released.
#[inline]
pub fn map_button_released<F>(event: SDL_MouseButtonEvent, mut f: F) -> Option<()>
where
    F: FnMut(Event),
{
    let button = map_button(event.button);

    f(Event::ButtonReleased(button));

    Some(())
}

/// Maps a `SDL_MouseMotionEvent` to a `Event`.
#[inline]
pub fn map_cursor_moved<F>(event: SDL_MouseMotionEvent, mut f: F) -> Option<()>
where
    F: FnMut(Event),
{
    let position = Point::new(event.x as f32, event.y as f32);

    f(Event::CursorMoved { position });

    Some(())
}
