#![deny(warnings)]

use iced_native::Event;
use sdl2_sys::{SDL_Event, SDL_EventType};

pub mod keyboard;
pub mod mouse;

const KEY_DOWN: u32 = SDL_EventType::SDL_KEYDOWN as u32;
const KEY_UP: u32 = SDL_EventType::SDL_KEYUP as u32;
const MOUSE_DOWN: u32 = SDL_EventType::SDL_MOUSEBUTTONDOWN as u32;
const MOUSE_UP: u32 = SDL_EventType::SDL_MOUSEBUTTONUP as u32;
const MOUSE_MOTION: u32 = SDL_EventType::SDL_MOUSEMOTION as u32;

pub fn map_event<F>(sdl_event: SDL_Event, mut f: F)
where
    F: FnMut(Event),
{
    unsafe {
        match sdl_event.type_ {
            KEY_DOWN => keyboard::map_key_pressed(sdl_event.key, |event| f(Event::Keyboard(event))),
            KEY_UP => keyboard::map_key_released(sdl_event.key, |event| f(Event::Keyboard(event))),
            MOUSE_DOWN => {
                mouse::map_button_pressed(sdl_event.button, |event| f(Event::Mouse(event)))
            }
            MOUSE_UP => {
                mouse::map_button_released(sdl_event.button, |event| f(Event::Mouse(event)))
            }
            MOUSE_MOTION => {
                mouse::map_cursor_moved(sdl_event.motion, |event| f(Event::Mouse(event)))
            }
            _ => None,
        };
    }
}
