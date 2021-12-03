use crate::console::Console;
use crate::engine::Engine;
use crate::entities::Entities;
use crate::entity::Entity;
use std::lazy::{SyncLazy, SyncOnceCell};
use std::ptr;

pub static CONSOLE: SyncOnceCell<Console> = SyncOnceCell::new();
pub static ENGINE: SyncOnceCell<Engine> = SyncOnceCell::new();
pub static ENTITIES: SyncOnceCell<Entities> = SyncOnceCell::new();
pub static LOCAL_PLAYER: SyncLazy<Entity> =
    SyncLazy::new(|| unsafe { Entity::from_raw(ptr::null()) });

pub unsafe fn console() -> &'static Console {
    CONSOLE.get().unwrap_unchecked()
}

pub unsafe fn engine() -> &'static Engine {
    ENGINE.get().unwrap_unchecked()
}

pub unsafe fn entities() -> &'static Entities {
    ENTITIES.get().unwrap_unchecked()
}

pub fn set_console(console: *const ()) {
    let _ = unsafe { CONSOLE.set(Console::from_raw(console)) };
}

pub fn set_engine(engine: *const ()) {
    let _ = unsafe { ENGINE.set(Engine::from_raw(engine)) };
}

pub fn set_entities(entities: *const ()) {
    let _ = unsafe { ENTITIES.set(Entities::from_raw(entities)) };
}

pub fn local_player() -> Option<&'static Entity> {
    if LOCAL_PLAYER.as_ptr().is_null() {
        None
    } else {
        Some(&LOCAL_PLAYER)
    }
}

pub fn set_local_player(entity: Entity) {
    unsafe {
        ptr::replace::<*const ()>(
            &LOCAL_PLAYER.this as *const *const () as *mut *const (),
            entity.this,
        );
    }
}

pub fn reset_local_player() {
    unsafe {
        ptr::replace::<*const ()>(
            &LOCAL_PLAYER.this as *const *const () as *mut *const (),
            ptr::null(),
        );
    }
}
