#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Mouse(u32);

impl Mouse {
    pub const RELEASED: Self = Self::new(0);
    pub const PRESSED: Self = Self::new(1);
    pub const DOUBLECLICKED: Self = Self::new(2);

    pub const COUNT: u32 = 5;

    const fn new(state: u32) -> Self {
        Self(state)
    }
}
