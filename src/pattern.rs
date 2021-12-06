use findshlibs::{Segment, SharedLibrary, TargetSharedLibrary};
use parking_lot::RwLock;
use regex::bytes::Regex;
use std::collections::HashMap;
use std::lazy::SyncLazy;
use std::mem::ManuallyDrop;
use std::path::Path;
use std::slice;
use std::sync::Arc;

macro_rules! pattern {
    ($pattern:expr) => {
        SyncLazy::new(move || Regex::new($pattern).unwrap())
    };
}

// tbh forgot x86s encoding is variable in len so the disasm is likely wrong in places
// esp after the ??'s

// 55        push rbp
// 48 89 e5  mov  rbp, rsp
// 41 55     push r13
// 49 89 fd  mov  r13, rdi
// 41 54     push r12
// bf
pub const SEND_CLANTAG: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x55\x48\x89\xe5\x41\x55\x49\x89\xfd\x41\x54\xbf");

// 55           push rbp
// 48 89 f7     mov  rdi, rsi
// 48 8d 35
// ?? ?? ?? ??
// 48 89 e5     mov  rbp, rsp
// ?? ?? ?? ??
// 85 c0        test eax, eax
pub const SET_PLAYER_READY: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x55\x48\x89\xf7\x48\x8d\x35....\x48\x89\xe5\xe8....\x85\xc0");

// 84 c0  test al, al
// 74 50  je   0x54
// xor    esi, esi
pub const RADAR_IS_HLTV_CHECK: SyncLazy<Regex> = pattern!(r"(?msx-u)\x84\xc0\x74\x50\x31\xf6");

// 81 27 00 00 00 ff  and  dword ptr [rdi], 0xff000000
// 55                 push rbp
// 31 c0              xor  eax, eax
// 48 89 45           mov  rbp, rsp
// 5d                 pop  rbp
pub const INIT_KEY_VALUES: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x81\x27\x00\x00\x00\xff\x55\x31\xc0\x48\x89\xe5\x5d");

// 55           push rbp
// 48 89 e5     mov  rbp, rsp
// 41 57        push r15
// 41 56        push r14
// 41 55        push r13
// 41 54        push r12
// 49 89 d4     mov  r12, rdx
// 53           push rbx
// 48 81 ec
// ?? ?? ?? ??
// 48 85
pub const LOAD_FROM_BUFFER: SyncLazy<Regex> = pattern!(
    r"(?msx-u)\x55\x48\x89\xe5\x41\x57\x41\x56\x41\x55\x41\x54\x49\x89\xd4\x53\x48\x81\xec....\x48\x85"
);

// 55           push rbp
// 4c 8d 05
// ?? ?? ?? ??
// 48 89 e5     mov rbp, rsp
// 41
pub const SET_NAMED_SKYBOX: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x55\x4c\x8d\x05....\x48\x89\xe5\x41");

// 55              push rbp
// 48 89 e5        mov  rbp, rsp
// 41 56           push r14
// 41 55           push r13
// 41 54           push r12
// 53              push rbx
// 48 83 ec 30     sub  rsp, 0x30
// 66 0f d6 45 d0  movq qword ptr [rbp - 0x30], xmm0
pub const LINE_GOES_THROUGH_SMOKE: SyncLazy<Regex> = pattern!(
    r"(?msx-u)\x55\x48\x89\xe5\x41\x56\x41\x55\x41\x54\x53\x48\x83\xec\x30\x66\x0f\xd6\x45\xd0"
);

// 48 8b 0d
// ?? ?? ??
pub const MOVE_DATA: SyncLazy<Regex> = pattern!(r"(?msx-u)\x48\x8b\x0d....\x4c\x89\xea");

pub const MOVE_HELPER: SyncLazy<Regex> = pattern!(r"(?msx-u)\x00\x48\x89\x3d....\xc3");

pub const PREDICTION_SEED: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x48\x8b\x05....\x8b\x38\xe8....\x89\xc7");

// 55         push rbp
// 48 89 e5   mov  rbp, rsp
// 41 56      push r14
// 41 55      push r13
// 41 89 f5   mov  r13d, esi
// 41 54      push r12
// 53         push rbx
// 48 98 fb   mov  rbx, rdi
// 8b
pub const ANIMATION_LAYERS: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x55\x48\x89\xe5\x41\x56\x41\x55\x41\x89\xf5\x41\x54\x53\x48\x89\xfb\x8b");

// 55          push rbp,
// 48 89 e5    mov  rbp, rsp
// 53          push rbx
// 48 89 fb    mov  rbx, rdi
// 48 83 ec 28 sub  rsp, 0x28
// 48 8b 05
// ?? ?? ?? ??
// 48 8b 0d
pub const ANIMATION_STATE: SyncLazy<Regex> = pattern!(
    r"(?msx-u)\x55\x48\x89\xe5\x53\x48\x89\xfb\x48\x83\xec\x28\x48\x8b\x05....\x48\x8b\x00"
);

// 55        push   rbp
// 48 89 e5  mov    rbp, rsp
// 41 57     push   r15
// 41 89 cf  mov    r15d, ecx
// 41 56     push   r14
// 41 55     push   r13
// 41 89 d5  mov    r13d, edx
// 41 54     push   r12
// 53        push   rbx
// 48 89 fb  mov    rbx, rdi
// 48 81 ec
pub const SAVE_DATA: SyncLazy<Regex> = pattern!(
    r"(?msx-u)\x55\x48\x89\xe5\x41\x57\x41\x89\xcf\x41\x56\x41\x55\x41\x89\xd5\x41\x54\x53\x48\x89\xfb\x48\x81\xec"
);

// x9
// ?? ?? ?? ??
// 90           nop
// 55           push   rbp
// 48 63 f6     movsxd rsi, esi
pub const RESTORE_DATA: SyncLazy<Regex> = pattern!(r"\xe9....\x90\x55\x48\x63\xf6");

// 55           push rbp
// be
// ?? ?? ?? ??
// 48 89 e5     mov  rbp, rsp
// 41 54        push r12
// 53           push rbx
// 48 89 fb     mov  rbx, rdi
// e8
pub const ON_POST_RESTORE_DATA: SyncLazy<Regex> =
    pattern!(r"(?msx-u)\x55\xbe....\x48\x89\xe5\x41\x54\x53\x48\x89\xfb\xe8");

// 55           push rbp
// 48 89 e5     mov  rbp, rsp
// 41 57        push r15
// 41 89 d7     mov  r15d, edx
// 41 56        push r14
// 41 55        push r13
// 41 89 f5     mov  r13d, esi
// 41 54        push r12
// 53           push rbx
// 48 83 ec 18  sub  rsp, 0x18
pub const RESTORE_ENTITY_TO_PREDICTED_FRAME: SyncLazy<Regex> = pattern!(
    r"(?msx-u)\x55\x48\x89\xe5\x41\x57\x41\x89\xd7\x41\x56\x41\x55\x41\x89\xf5\x41\x54\x53\x48\x83\xec\x18"
);

#[derive(Debug)]
pub struct Libraries(pub Arc<RwLock<HashMap<Box<str>, (usize, ManuallyDrop<Box<[u8]>>)>>>);

impl Libraries {
    pub fn new() -> Self {
        let libs = Arc::new(RwLock::new(HashMap::new()));
        let libs2 = libs.clone();

        TargetSharedLibrary::each(move |lib| {
            if let Some(phdr) = lib.segments().next() {
                let name = lib.name().to_string_lossy();

                if !name.contains("Counter-Strike Global Offensive") {
                    return;
                }

                let name = Path::new(name.as_ref());
                let name = match name.file_name() {
                    Some(file_name) => file_name,
                    None => return,
                };

                let name = name.to_string_lossy().into_owned().into_boxed_str();

                let address = lib.virtual_memory_bias().0 + phdr.stated_virtual_memory_address().0;
                let len = phdr.len();

                println!("{} {} {}", &name, &address, &len);

                let slice =
                    unsafe { slice::from_raw_parts(address as *const usize as *const u8, len) };
                let slice = ManuallyDrop::new(Box::<[u8]>::from(slice));

                libs2.write().insert(name, (address, slice));
            }
        });

        Self(libs)
    }
}
