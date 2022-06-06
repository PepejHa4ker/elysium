use core::ptr;
use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use providence_pattern::Pattern;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

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
pub const WRITE_USER_COMMAND: Pattern<68> =
    Pattern::new("55 48 89 E5 41 56 41 55 4C 8D 35 B1 19 17 02");

/// xref `WriteUsercmd`
pub const WRITE_USER_COMMAND_DELTA_TO_BUFFER: Pattern<72> =
    Pattern::new("55 48 8D 05 38 BC 68 01 41 89 F2 48 89 E5 41 57");

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

            // we're only interested in the library names themselves
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
        name: &str,
    ) -> Option<*const u8> {
        self.read().address_of(library_name, pattern).map(|address| {
            println!("elysium | found pattern \x1b[38;5;2m{pattern:?}\x1b[m (\x1b[38;5;2m{name}\x1b[m) within \x1b[38;5;2m{library_name}\x1b[m at \x1b[38;5;3m{address:?}\x1b[m");

            address
        })
    }
}
