pub use cached_player::CachedPlayer;
use core::mem::MaybeUninit;
use core::ptr;
use providence_math::Vec3;
use providence_model::Bones;
use shared::Shared;

mod cached_player;
mod shared;

pub type CreateMove =
    unsafe extern "C" fn(this: *const (), sample_time: f32, command: *const ()) -> bool;

pub struct CachedPlayers {
    pub players: [CachedPlayer; 64],
}

impl CachedPlayers {
    const INIT: CachedPlayer = CachedPlayer::new();

    pub const fn new() -> CachedPlayers {
        let players = [Self::INIT; 64];

        Self { players }
    }
}

struct State {
    player: *mut (),
    weapon: *mut (),
    magazine_ammo: i32,
    total_ammo: i32,
    health: i32,

    create_move: MaybeUninit<CreateMove>,
    cached_players: CachedPlayers,
    local_player_bones: Bones,
    prediction_time: f32,
    send_packet: *mut bool,
    tick_count: i32,
    view_angle: Vec3,
    original_view_angle: Vec3,
}

static STATE: Shared<State> = Shared::new(State {
    player: ptr::null_mut(),
    weapon: ptr::null_mut(),
    magazine_ammo: 0,
    total_ammo: 0,
    health: 0,

    cached_players: CachedPlayers::new(),
    create_move: MaybeUninit::uninit(),
    local_player_bones: Bones::zero(),
    original_view_angle: Vec3::splat(0.0),
    prediction_time: 0.0,
    send_packet: ptr::null_mut(),
    tick_count: 0,
    view_angle: Vec3::splat(0.0),
});

#[inline]
pub unsafe fn cached_players() -> *mut CachedPlayers {
    &mut STATE.as_mut().cached_players
}

#[inline]
pub unsafe fn create_move(this: *const (), sample_time: f32, command: *const ()) -> bool {
    (STATE.as_mut().create_move.assume_init())(this, sample_time, command)
}

#[inline]
pub unsafe fn set_create_move(create_move: CreateMove) {
    STATE.as_mut().create_move = MaybeUninit::new(create_move);
}

#[inline]
pub unsafe fn local_player_bones() -> *mut Bones {
    &mut STATE.as_mut().local_player_bones
}

#[inline]
pub unsafe fn original_view_angle() -> *mut Vec3 {
    &mut STATE.as_mut().original_view_angle
}

#[inline]
pub unsafe fn prediction_time() -> *mut f32 {
    &mut STATE.as_mut().prediction_time
}

#[inline]
pub unsafe fn send_packet() -> *mut *mut bool {
    &mut STATE.as_mut().send_packet
}

#[inline]
pub unsafe fn tick_count() -> *mut i32 {
    &mut STATE.as_mut().tick_count
}

#[inline]
pub unsafe fn view_angle() -> *mut Vec3 {
    &mut STATE.as_mut().view_angle
}
