use crate::state;
use iced_native::keyboard::Event::{KeyPressed, KeyReleased};
use iced_native::keyboard::KeyCode::Insert;
use iced_native::mouse::Button::Other;
use iced_native::mouse::Event::{ButtonPressed, ButtonReleased};
use iced_native::{mouse, Event};

/// `SDL_PollEvent` hook.
pub unsafe extern "C" fn poll_event(sdl_event: *mut sdl2_sys::SDL_Event) -> i32 {
    let result = state::hooks::poll_event(sdl_event);

    if !state::is_menu_none() {
        let menu = state::menu_unchecked();

        elysium_input::map_event(*sdl_event, |event| {
            match &event {
                // insert
                Event::Keyboard(KeyPressed {
                    key_code: Insert, ..
                }) => state::toggle_menu(),

                // thirdperson
                Event::Mouse(ButtonPressed(Other(4))) => state::local::toggle_thirdperson(),

                // p100 duplicate input fixes
                // insert
                Event::Keyboard(KeyReleased {
                    key_code: Insert, ..
                }) => state::release_toggle_menu(),

                // thirdperson
                Event::Mouse(ButtonReleased(Other(4))) => {
                    state::local::release_toggle_thirdperson()
                }

                // move cursor
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    // block input to the game when the menu is open
    if state::is_menu_open() {
        (*sdl_event).type_ = 0;
    }

    result
}
