use std::mem;

#[derive(Debug)]
#[repr(i32)]
pub enum FrameStage {
    Undefined = -1,
    Start,
    NetUpdateStart,
    NetUpdatePostDataUpdateStart,
    NetUpdatePostDataUpdateEnd,
    NetUpdateEnd,
    RenderStart,
    RenderEnd,
}

impl FrameStage {
    /// Create a FrameStage.
    pub fn new(stage: i32) -> Option<Self> {
        use FrameStage::*;

        let stage = match stage {
            -1 => Undefined,
            0 => Start,
            1 => NetUpdateStart,
            2 => NetUpdatePostDataUpdateStart,
            3 => NetUpdatePostDataUpdateEnd,
            4 => NetUpdateEnd,
            5 => RenderStart,
            6 => RenderEnd,
            _ => return None,
        };

        Some(stage)
    }

    /// Create a FrameStage without checking.
    pub unsafe fn from_raw(stage: i32) -> Self {
        mem::transmute(stage)
    }
}
