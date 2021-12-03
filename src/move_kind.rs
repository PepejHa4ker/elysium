#[repr(i32)]
pub enum MoveKind {
    None = 0,
    Isometric,
    Walk,
    Step,
    Fly,
    FlyGravity,
    VPhysics,
    Push,
    NoClip,
    Ladder,
    Observer,
    Custom,
}

impl MoveKind {
    /// If this movement is eligible for bunny hopping.
    pub const fn can_bhop(&self) -> bool {
        use MoveKind::*;

        !matches!(self, Ladder | NoClip)
    }
}
