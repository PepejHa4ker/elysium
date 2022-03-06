use crate::entity::Player;
use providence_math::Vec3;

#[derive(Debug)]
pub struct Movement {
    /// Forward, side, and up movement vectors.
    pub vectors: Vec3,

    /// Current view angle.
    pub view: Vec3,

    pub command_number: i32,
    pub tick_count: i32,
    pub send_packet: bool,
    pub do_attack: bool,
    pub do_jump: bool,
    pub do_duck: bool,
    pub do_fast_duck: bool,
    pub do_left: bool,
    pub do_right: bool,
    pub local_player: Player,
    pub client_time: f32,
    pub prediction_time: f32,
    pub server_time: f32,
}
