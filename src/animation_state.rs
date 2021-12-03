use sdk::{Angle, Vector2D};

#[derive(Debug)]
#[repr(C)]
pub struct AnimationState {
    _pad_0008: [u8; 128],
    pub last_animation_update_time: f32,
    pub last_animation_update_frame: i32,
    pub eye_pitch: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub goal_feet_yaw: f32,
    pub current_feet_yaw: f32,
    /// Movement direction on a compass.
    ///
    ///   North: `0`
    ///   East: `90`
    ///   South: `180`
    ///   West: `270`
    pub absolute_movement_direction: f32,
    /// The value of `absolute_movement_direction` in the last tick.
    pub last_absolute_movement_direction: f32,
    pub lean_amount: f32,
    _pad_00A8: [u8; 4],
    /// Progress from `0.0` to `1.0` of the legs moving animation.
    pub feet_cycle: f32,
    /// How fast to play `feet_cycle`.
    ///
    /// Walking is slower. Running is faster.
    pub feet_yaw_rate: f32,
    _pad_00B4: [u8; 4],
    /// Progress from `0.0` to `1.0` of the crouch and jump animations.
    ///
    /// For crouching:
    ///
    ///  Standing: `0.0`
    ///  Crouched: `1.0`
    ///
    /// For jumping:
    ///  
    ///  Jumpped: `1.0`
    ///  Standing: `0.0`
    pub duck_progress: f32,
    /// Adds some time when landing animation starts.
    pub landing_animation_time_left_until_done: f32,
    _pad_00C0: [u8; 4],
    pub origin: Angle,
    pub last_origin: Angle,
    /// Velocity on each axis.
    pub axis_velocity: Vector2D,
    _pad_00E4: [u8; 4],
    /// Both axis are `-1.0` to `1.0`.
    ///
    /// For `x`, west is `1.0`, east is `-1.0`.
    /// For `y`, north is `-1.0`, south is `1.0`.
    pub current_direction: Vector2D,
    _pad_00F0: [u8; 4],
    /// Same as `current_direction`. Saved until the next update,
    pub last_known_direction: Vector2D,
    _pad_00FC: [u8; 4],
    /// Velocity on each plane.
    ///
    /// `x` is affected by slopes. (e.g. 240 with a knife on a hill).
    /// `y` positive when moving up (e.g. jumping), negative when moving down (e.g. falling).
    pub plane_velocity: Vector2D,
    /// Speed normalized in terms of `0.0 to `1.0`.
    ///
    /// Full speed: `1.0`.
    pub speed: f32,
    /// `0.0` to `2.0` of the aprroximate feet shuffle speed.
    pub feet_shuffle_speed: f32,
    /// `0.0` to `2.0` of the aprroximate feet shuffle speed 2.
    pub feet_shuffle_speed2: f32,
    /// Jumping and crouching does not affect this.
    pub time_since_started_moving: f32,
    /// Jumping and crouching does not affect this.
    pub time_since_stopped_moving: f32,
    pub on_ground: bool,
    /// `true` if performing a knee buckling animation.
    pub in_jump_recovery: bool,
    _pad_011E: [u8; 10],
    /// Updated right before you jump.
    pub height_before_jump: [u8; 10],
    _pad_012C: [u8; 4],
    /// Running progeess from `0.0` to `1.0`.
    ///
    /// Only affected by running.
    /// Full speed: `1.0` (e.g. 250 velocity when running with knife).
    pub running_accel_progress: f32,
    _pad_0134: [u8; 68],
    // Overall size should be 0x3B0(+4), padding the end here.
    _unknown: [u8; 572],
} // size: 0x0178
