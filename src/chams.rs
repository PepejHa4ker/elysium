enum Kind {
    Default,
    None,
    Flat,
    Shaded,
    ScreenPulse,
    EnergyBall,
    Glow,
    Plastic,
    Darude,
    Oil,
    Platinum,
    Crystal,
    Silver,
    Glass,
    Gold,
}

struct Layer {
    kind: Kind,
    ignore_z: bool,
    wireframe: bool,
}

pub struct Chams {
    default_layer: Layer,
    second_layer: Layer,
}

impl Chams {
    /// Default rendering.
    pub const fn new() -> Self {
        Self {
            kind: Kind::Default,
            second_kind: Kind::Default,
        }
    }
}
