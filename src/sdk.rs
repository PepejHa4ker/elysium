use vek::vec::repr_c::{Vec2, Vec3};

pub mod animation_state;
pub mod client;
pub mod command;
pub mod console;
pub mod engine;
pub mod engine_trace;
pub mod entities;
pub mod entity;
pub mod entity_id;
pub mod frame_stage;
pub mod hit_group;
pub mod input;
pub mod item_kind;
pub mod move_kind;
pub mod netvars;
pub mod player_state;
pub mod skybox;

pub type Vec2f32 = Vec2<f32>;
pub type Vec3f32 = Vec3<f32>;

pub type Angle = Vec3f32;
pub type Vector = Vec3f32;

pub use self::client::{Client, ClientClass, RecvProp, RecvTable};
pub use self::command::Command;
pub use self::console::Console;
pub use self::engine::Engine;
pub use self::engine_trace::EngineTrace;
pub use self::entities::Entities;
pub use self::entity::Entity;
pub use self::entity_id::EntityId;
pub use self::frame_stage::FrameStage;
pub use self::player_state::PlayerState;
