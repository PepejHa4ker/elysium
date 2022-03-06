use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use providence_math::Vec3;

struct Shared<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> Shared<T> {
    pub const fn uninit() -> Self {
        Self(UnsafeCell::new(MaybeUninit::uninit()))
    }

    pub unsafe fn as_ref(&self) -> &T {
        &*(self.0.get() as *const T)
    }

    pub unsafe fn as_mut(&self) -> &mut T {
        &mut *(self.0.get() as *mut T)
    }
}

unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}

struct State {
    prediction_time: f32,
    view_angle: Vec3,
}

static STATE: Shared<State> = Shared::uninit();

pub unsafe fn prediction_time() -> f32 {
    STATE.as_ref().prediction_time
}

pub unsafe fn set_prediction_time(prediction_time: f32) {
    STATE.as_mut().prediction_time = prediction_time;
}

pub unsafe fn view_angle() -> Vec3 {
    STATE.as_ref().view_angle
}

pub unsafe fn set_view_angle(view_angle: Vec3) {
    STATE.as_mut().view_angle = view_angle;
}
