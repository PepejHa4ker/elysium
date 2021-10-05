use crate::sdk;
use parking_lot::{RwLock, RwLockReadGuard};
use std::lazy::SyncOnceCell;
use std::mem;

pub static CONSOLE: SyncOnceCell<sdk::Console> = SyncOnceCell::new();
pub static ENGINE: SyncOnceCell<sdk::Engine> = SyncOnceCell::new();
pub static ENTITIES: SyncOnceCell<sdk::Entities> = SyncOnceCell::new();
pub static LOCAL_PLAYER: SyncOnceCell<RwLock<sdk::Entity>> = SyncOnceCell::new();

pub unsafe fn console() -> &'static sdk::Console {
    CONSOLE.get().unwrap_unchecked()
}

pub unsafe fn engine() -> &'static sdk::Engine {
    ENGINE.get().unwrap_unchecked()
}

pub unsafe fn entities() -> &'static sdk::Entities {
    ENTITIES.get().unwrap_unchecked()
}

pub fn set_console(console: *const usize) {
    let _ = unsafe { CONSOLE.set(sdk::Console::from_raw(console)) };
}

pub fn set_engine(engine: *const usize) {
    let _ = unsafe { ENGINE.set(sdk::Engine::from_raw(engine)) };
}

pub fn set_entities(entities: *const usize) {
    let _ = unsafe { ENTITIES.set(sdk::Entities::from_raw(entities)) };
}

pub unsafe fn local_player_unchecked<'a>() -> RwLockReadGuard<'a, sdk::Entity> {
    LOCAL_PLAYER.get().unwrap_unchecked().read()
}

pub fn local_player<'a>() -> Option<RwLockReadGuard<'a, sdk::Entity>> {
    LOCAL_PLAYER.get().map(|local_player| local_player.read())
}

pub fn set_local_player(entity: sdk::Entity) {
    let _ = LOCAL_PLAYER.set(RwLock::new(entity));
}
