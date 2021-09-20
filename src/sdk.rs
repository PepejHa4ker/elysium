use vek::vec::repr_c::{Vec2, Vec3};

pub mod animation_state;
pub mod entity_id;
pub mod frame_stage;
pub mod hit_group;
pub mod input;
pub mod item_kind;
pub mod move_kind;
pub mod player_state;
pub mod skybox;

pub type Vec2f32 = Vec2<f32>;
pub type Vec3f32 = Vec3<f32>;

pub type Angle = Vec3f32;
pub type Vector = Vec3f32;
