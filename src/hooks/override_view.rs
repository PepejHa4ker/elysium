use crate::state;
use elysium_sdk::View;

/// `OverrideView` hook.
pub unsafe extern "C" fn override_view(this: *const u8, view: *mut u8) {
    let view = &mut *view.cast::<View>();

    view.angle = *state::view_angle();

    state::hooks::override_view(this, (view as *mut View).cast());
}
