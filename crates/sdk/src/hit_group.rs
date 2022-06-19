#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum HitGroup {
    Generic = 0,
    Head = 1,
    Chest = 2,
    Stomach = 3,
    LeftArm = 4,
    RightArm = 5,
    LeftLeg = 6,
    RightLeg = 7,
    Gear = 8,
}

impl HitGroup {
    /// returns the damage modifier for the given hit group
    #[inline]
    pub const fn damage_modifier(&self) -> f32 {
        match self {
            HitGroup::Head => 4.0,
            HitGroup::Stomach => 1.25,
            HitGroup::LeftLeg | HitGroup::RightLeg => 0.75,
            _ => 1.0,
        }
    }

    // if the hit group is the head
    #[inline]
    pub const fn is_head(&self) -> bool {
        matches!(self, HitGroup::Head)
    }
}
