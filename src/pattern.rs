use core::ptr;
use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use regex::bytes::Regex;
use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::path::Path;
use std::sync::Arc;

pub type Pattern = SyncLazy<Regex>;

macro_rules! pattern {
    ($pattern:expr) => {
        SyncLazy::new(move || Regex::new(concat!("(?msx-u)", $pattern)).unwrap())
    };
}

pub const ITEM_SYSTEM: Pattern = pattern!(r"\xe8....\x4d\x63\xec");

pub const WEAPON_SYSTEM: Pattern = pattern!(r"\x48\x8b\x58\x10\x48\x8b\x07\xff\x10");

pub const SEND_CLANTAG: Pattern = pattern!(r"\x55\x48\x89\xe5\x41\x55\x49\x89\xfd\x41\x54\xbf");

pub const SET_PLAYER_READY: Pattern =
    pattern!(r"\x55\x48\x89\xf7\x48\x8d\x35....\x48\x89\xe5\xe8....\x85\xc0");

pub const RADAR_IS_HLTV_CHECK: Pattern = pattern!(r"\x84\xc0\x74\x50\x31\xf6");

pub const INIT_KEY_VALUES: Pattern =
    pattern!(r"\x81\x27\x00\x00\x00\xff\x55\x31\xc0\x48\x89\xe5\x5d");

pub const LOAD_FROM_BUFFER: Pattern = pattern!(
    r"\x55\x48\x89\xe5\x41\x57\x41\x56\x41\x55\x41\x54\x49\x89\xd4\x53\x48\x81\xec....\x48\x85"
);

pub const SET_NAMED_SKYBOX: Pattern = pattern!(r"\x55\x4c\x8d\x05....\x48\x89\xe5\x41");

pub const LINE_GOES_THROUGH_SMOKE: Pattern =
    pattern!(r"\x55\x48\x89\xe5\x41\x56\x41\x55\x41\x54\x53\x48\x83\xec\x30\x66\x0f\xd6\x45\xd0");

pub const MOVE_DATA: Pattern = pattern!(r"\x48\x8b\x0d....\x4c\x89\xea");

pub const MOVE_HELPER: Pattern = pattern!(r"\x00\x48\x89\x3d....\xc3");

pub const PREDICTION_SEED: Pattern = pattern!(r"\x48\x8b\x05....\x8b\x38\xe8....\x89\xc7");

pub const ANIMATION_LAYERS: Pattern =
    pattern!(r"\x55\x48\x89\xe5\x41\x56\x41\x55\x41\x89\xf5\x41\x54\x53\x48\x89\xfb\x8b");

pub const ANIMATION_STATE: Pattern =
    pattern!(r"\x55\x48\x89\xe5\x53\x48\x89\xfb\x48\x83\xec\x28\x48\x8b\x05....\x48\x8b\x00");

pub const SAVE_DATA: Pattern = pattern!(
    r"\x55\x48\x89\xe5\x41\x57\x41\x89\xcf\x41\x56\x41\x55\x41\x89\xd5\x41\x54\x53\x48\x89\xfb\x48\x81\xec"
);

pub const RESTORE_DATA: Pattern = pattern!(r"\xe9....\x90\x55\x48\x63\xf6");

pub const ON_POST_RESTORE_DATA: Pattern =
    pattern!(r"\x55\xbe....\x48\x89\xe5\x41\x54\x53\x48\x89\xfb\xe8");

pub const RESTORE_ENTITY_TO_PREDICTED_FRAME: Pattern = pattern!(
    r"\x55\x48\x89\xe5\x41\x57\x41\x89\xd7\x41\x56\x41\x55\x41\x89\xf5\x41\x54\x53\x48\x83\xec\x18"
);

/// non-owning range over some memory
#[derive(Clone, Copy, Debug)]
pub struct Range {
    base_address: *const u8,
    len: usize,
}

impl Range {
    pub const fn new(base_address: *const u8, len: usize) -> Self {
        Self { base_address, len }
    }

    pub const unsafe fn as_slice(&self) -> &[u8] {
        &*ptr::from_raw_parts(self.base_address as *const (), self.len)
    }

    pub unsafe fn offset_of(&self, pattern: &Pattern) -> Option<usize> {
        match pattern.find(self.as_slice()) {
            Some(r#match) => Some(r#match.start()),
            None => None,
        }
    }

    pub unsafe fn address_of(&self, pattern: &Pattern) -> Option<*const u8> {
        match self.offset_of(pattern) {
            Some(offset) => Some(self.base_address.add(offset)),
            None => None,
        }
    }
}

pub struct Ranges {
    ranges: HashMap<Box<str>, Range>,
}

impl Ranges {
    pub fn new() -> Self {
        Self {
            ranges: HashMap::new(),
        }
    }

    pub fn insert(&mut self, library_name: &str, base_address: *const u8, len: usize) {
        let range = Range::new(base_address, len);

        self.ranges.insert(library_name.into(), range);
    }

    pub fn get(&self, library_name: &str) -> Option<Range> {
        match self.ranges.get(library_name) {
            Some(range) => Some(*range),
            None => None,
        }
    }

    pub unsafe fn offset_of(&self, library_name: &str, pattern: &Pattern) -> Option<usize> {
        match self.get(library_name) {
            Some(range) => range.offset_of(pattern),
            None => None,
        }
    }

    pub unsafe fn address_of(&self, library_name: &str, pattern: &Pattern) -> Option<*const u8> {
        match self.get(library_name) {
            Some(range) => range.address_of(pattern),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Libraries(pub Arc<RwLock<Ranges>>);

impl Libraries {
    pub fn new() -> Self {
        let this = Self(Arc::new(RwLock::new(Ranges::new())));
        let this2 = this.clone();

        TargetSharedLibrary::each(move |library| {
            // skip libraries without a program header
            let program_header = match library.segments().next() {
                Some(program_header) => program_header,
                None => return,
            };

            let library_name = library.name().to_string_lossy();

            // skip libraries that dont belong to csgo
            if !library_name.contains("Counter-Strike Global Offensive") {
                return;
            }

            // were only interested in the library names themselves
            let library_name = Path::new(library_name.as_ref());
            let library_name = match library_name.file_name() {
                Some(library_name) => library_name,
                None => return,
            };

            let library_name = library_name.to_string_lossy().into_owned().into_boxed_str();

            // library's base address and length
            let base_address =
                library.virtual_memory_bias().0 + program_header.stated_virtual_memory_address().0;
            let len = program_header.len();

            this2.insert(&library_name, base_address as *const u8, len);
        });

        this
    }

    fn read(&self) -> RwLockReadGuard<'_, Ranges> {
        self.0.read()
    }

    fn write(&self) -> RwLockWriteGuard<'_, Ranges> {
        self.0.write()
    }

    fn insert(&self, library_name: &str, base_address: *const u8, len: usize) {
        self.write().insert(library_name, base_address, len);
    }

    /*fn get(&self, library_name: &str) -> Option<Range> {
        self.read().get(library_name)
    }*/

    pub unsafe fn offset_of(&self, library_name: &str, pattern: &Pattern) -> Option<usize> {
        self.read().offset_of(library_name, pattern)
    }

    pub unsafe fn address_of(&self, library_name: &str, pattern: &Pattern) -> Option<*const u8> {
        self.read().address_of(library_name, pattern)
    }
}
