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
    pub in_duck: bool,
    /// you may be vac banned using this
    pub in_fast_duck: bool,
    pub local_player: Entity,
}
