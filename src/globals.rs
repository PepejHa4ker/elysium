use crate::sdk;
//use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::lazy::{SyncLazy, SyncOnceCell};
use std::ptr;

pub static CONSOLE: SyncOnceCell<sdk::Console> = SyncOnceCell::new();
pub static ENGINE: SyncOnceCell<sdk::Engine> = SyncOnceCell::new();
pub static ENGINE_TRACE: SyncOnceCell<sdk::EngineTrace> = SyncOnceCell::new();
pub static ENTITIES: SyncOnceCell<sdk::Entities> = SyncOnceCell::new();
pub static LOCAL_PLAYER: SyncLazy<sdk::Entity> =
    SyncLazy::new(|| unsafe { sdk::Entity::from_raw(ptr::null()) });

pub unsafe fn console() -> &'static sdk::Console {
    CONSOLE.get().unwrap_unchecked()
}

pub unsafe fn engine() -> &'static sdk::Engine {
    ENGINE.get().unwrap_unchecked()
}

pub unsafe fn engine_trace() -> &'static sdk::EngineTrace {
    ENGINE_TRACE.get().unwrap_unchecked()
}

pub unsafe fn entities() -> &'static sdk::Entities {
    ENTITIES.get().unwrap_unchecked()
}

pub fn set_console(console: *const ()) {
    let _ = unsafe { CONSOLE.set(sdk::Console::from_raw(console)) };
}

pub fn set_engine(engine: *const ()) {
    let _ = unsafe { ENGINE.set(sdk::Engine::from_raw(engine)) };
}

pub fn set_engine_trace(engine_trace: *const ()) {
    let _ = unsafe { ENGINE_TRACE.set(sdk::EngineTrace::from_raw(engine_trace)) };
}

pub fn set_entities(entities: *const ()) {
    let _ = unsafe { ENTITIES.set(sdk::Entities::from_raw(entities)) };
}

pub fn local_player() -> Option<&'static sdk::Entity> {
    if LOCAL_PLAYER.as_ptr().is_null() {
        None
    } else {
        Some(&LOCAL_PLAYER)
    }
}

pub fn set_local_player(entity: sdk::Entity) {
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
