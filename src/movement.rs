use crate::entity::Player;
use sdk::Angle;

#[derive(Debug)]
pub struct Movement {
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub view_angle: Angle,
    pub tick_count: i32,
    pub send_packet: bool,
    pub do_attack: bool,
    pub do_jump: bool,
    pub do_duck: bool,
    pub do_fast_duck: bool,
    pub local_player: Player,
    pub client_time: f32,
    pub prediction_time: f32,
    pub server_time: f32,
}
