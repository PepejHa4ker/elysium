use crate::state;

/// `SDL_PollEvent` hook.
pub unsafe extern "C" fn poll_event(sdl_event: *mut sdl2_sys::SDL_Event) -> i32 {
    let result = state::hooks::poll_event(sdl_event);

    let mut toggle_menu = false;
    let mut toggle_thirdperson = false;

    if !state::is_menu_none() {
        let menu = state::menu_unchecked();

        elysium_input::map_event(*sdl_event, |event| {
            use iced_native::{keyboard, mouse, Event};

            println!("{event:?}");

            match &event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Insert,
                    ..
                }) => toggle_menu = true,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Other(4))) => {
                    toggle_thirdperson = true
                }
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    state::update_cursor_position(*position)
                }
                _ => {}
            };

            menu.queue_event(event)
        });
    }

    if toggle_menu {
        state::toggle_menu();
    }

    if toggle_thirdperson {
        state::local::toggle_thirdperson();
    }

    // block input to the game when the menu is open
    if state::is_menu_open() {
        (*sdl_event).type_ = 0;
    }

    result
}
