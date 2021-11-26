pub trait FloatExt {
    fn normalize_pitch(self) -> Self;
    fn normalize_yaw(self) -> Self;

    fn forward() -> Self;
    fn backward() -> Self;
    fn left() -> Self;
    fn right() -> Self;

    fn up() -> Self;
    fn down() -> Self;
}

impl const FloatExt for f32 {
    /// clamp pitch between -89.0 and 89.0
    fn normalize_pitch(self) -> Self {
        self % 89.0
    }

    /// clamp yaw between -180.0 and 180.0
    fn normalize_yaw(self) -> Self {
        self % 180.0
    }

    fn forward() -> Self {
        0.0
    }

    fn backward() -> Self {
        -180.0
    }

    fn left() -> Self {
        -90.0
    }

    fn right() -> Self {
        90.0
    }

    fn up() -> Self {
        89.0
    }

    fn down() -> Self {
        -89.0
    }
}
