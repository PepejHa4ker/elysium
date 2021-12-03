use crate::entity::Entity;
use sdk::Angle;

#[derive(Debug)]
pub struct Movement {
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub view_angle: Angle,
    pub tick_count: i32,
    pub send_packet: bool,
    pub in_jump: bool,
    pub local_player: Entity,
}
