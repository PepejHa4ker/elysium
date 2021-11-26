#[derive(Debug)]
#[repr(i32)]
pub enum HitGroup {
    Generic = 0,
    Head,
    Chest,
    Stomach,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    Gear,
}

impl HitGroup {
    /// The multipler applied to this hit group.
    pub const fn damage_multiplier(&self) -> f32 {
        use HitGroup::*;

        match self {
            Head => 4.0,
            Stomach => 1.25,
            LeftLeg | RightLeg => 0.75,
            _ => 1.0,
        }
    }
}
