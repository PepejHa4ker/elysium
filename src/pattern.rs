use core::ptr;
use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use providence_pattern::Pattern;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/*pub const ITEM_SYSTEM: Pattern<5> = Pattern::new("E8 ?? ?? ?? ?? 4D 63 EC");

pub const WEAPON_SYSTEM: Pattern<5> = Pattern::new("48 8B 58 10 48 8B 07 FF 10");

pub const SEND_CLANTAG: Pattern<5> = Pattern::new("55 48 89 E5 41 55 49 89 FD 41 54 BF");

pub const SET_PLAYER_READY: Pattern<5> =
    Pattern::new("55 48 89 F7 48 8D 35 ?? ?? ?? ?? 48 89 E5 E8 ?? ?? ?? ?? 85 C0");

pub const RADAR_IS_HLTV_CHECK: Pattern<5> = Pattern::new("84 C0 74 50 31 F6");

pub const INIT_KEY_VALUES: Pattern<5> = Pattern::new("81 27 00 00 00 ff 55 31 c0 48 89 e5 5d");

pub const LOAD_FROM_BUFFER: Pattern<5> =
    Pattern::new("55 48 89 E5 41 57 41 56 41 55 41 54 49 89 D4 53 48 81 EC ?? ?? ?? ?? 48 85");

pub const SET_NAMED_SKYBOX: Pattern<5> = Pattern::new("55 4C 8D 05 ?? ?? ?? ?? 48 89 E5 41");

pub const LINE_GOES_THROUGH_SMOKE: Pattern<5> =
    Pattern::new("55 48 89 E5 41 56 41 55 41 54 53 48 83 EC 30 66 0F D6 45 D0");

pub const MOVE_DATA: Pattern<5> = Pattern::new("48 8b 0d ?? ?? ?? ?? 4c 89 ea");

pub const MOVE_HELPER: Pattern<5> = Pattern::new("00 48 89 3d ?? ?? ?? ?? c3");

pub const PREDICTION_SEED: Pattern<5> =
    Pattern::new("48 8B 05 ?? ?? ?? ?? 8B 38 E8 ?? ?? ?? ?? 89 C7");*/

pub const ANIMATION_LAYERS: Pattern<80> =
    Pattern::new("55 48 89 E5 41 56 41 55 41 89 F5 41 54 53 48 89 FB 8B");

pub const ANIMATION_STATE: Pattern<84> =
    Pattern::new("55 48 89 E5 53 48 89 FB 48 83 EC 28 48 8B 05 ?? ?? ?? ?? 48 8B 00");

pub const SAVE_DATA: Pattern<108> =
    Pattern::new("55 48 89 E5 41 57 41 89 CF 41 56 41 55 41 89 D5 41 54 53 48 89 FB 48 81 EC");

pub const RESTORE_DATA: Pattern<36> = Pattern::new("E9 ?? ?? ?? ?? 90 55 48 63 F6");

pub const ON_POST_RESTORE_DATA: Pattern<60> =
    Pattern::new("55 BE ?? ?? ?? ?? 48 89 E5 41 54 53 48 89 FB E8");

/// xref `"CL_Move"` in `VPROF` macro
///
/// [engine/cl_main.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/engine/cl_main.cpp)
pub const CL_MOVE: Pattern<132> = Pattern::new(
    "55 48 89 E5 41 57 41 56 41 89 FE 41 55 41 54 53 48 81 EC 98 01 00 00 F3 0F 11 85 5C FE FF FF",
);

/// xref `CL_Move`
///
/// [engine/host.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/engine/host.cpp)
pub const HOST_RUN_FRAME_INPUT: Pattern<164> =
    Pattern::new("55 48 89 E5 41 57 66 41 0F 7E C7 41 56 41 55 41 89 FD 41 54 53 48 83 EC 08 48 8B 1D C8 25 94 00 44 8B 83 0C 10 00 00");

/// xref `"WriteUsercmd: from=%d to=%d\"`
///
/// [game/shared/usercmd.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/game/shared/usercmd.cpp)
pub const WRITE_USER_COMMAND: Pattern<88> =
    Pattern::new("55 89 E5 57 56 53 83 EC 2C A1 DC 8E 53 01 8B 7D 08 8B 75 0C");

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

    pub unsafe fn offset_of<const N: usize>(&self, pattern: &Pattern<N>) -> Option<usize> {
        match pattern.regex().find(self.as_slice()) {
            Some(r#match) => Some(r#match.start()),
            None => None,
        }
    }

    pub unsafe fn address_of<const N: usize>(&self, pattern: &Pattern<N>) -> Option<*const u8> {
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

    pub unsafe fn offset_of<const N: usize>(
        &self,
        library_name: &str,
        pattern: &Pattern<N>,
    ) -> Option<usize> {
        match self.get(library_name) {
            Some(range) => range.offset_of(pattern),
            None => None,
        }
    }

    pub unsafe fn address_of<const N: usize>(
        &self,
        library_name: &str,
        pattern: &Pattern<N>,
    ) -> Option<*const u8> {
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

    pub unsafe fn offset_of<const N: usize>(
        &self,
        library_name: &str,
        pattern: &Pattern<N>,
    ) -> Option<usize> {
        self.read().offset_of(library_name, pattern)
    }

    pub unsafe fn address_of<const N: usize>(
        &self,
        library_name: &str,
        pattern: &Pattern<N>,
    ) -> Option<*const u8> {
        self.read().address_of(library_name, pattern)
    }
}
