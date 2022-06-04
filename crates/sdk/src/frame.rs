/// argument passed to `FrameStageNotify`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Frame {
    #[doc(alias = "FRAME_UNDEFINED")]
    #[doc(alias = "UNDEFINED")]
    Undefined = -1,

    #[doc(alias = "FRAME_START")]
    #[doc(alias = "START")]
    Start = 0,

    #[doc(alias = "FRAME_NET_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_START")]
    UpdateStart = 1,

    #[doc(alias = "FRAME_NET_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_END")]
    UpdateEnd = 4,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_START")]
    PostDataStart = 2,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_END")]
    PostDataEnd = 3,

    #[doc(alias = "FRAME_RENDER_START")]
    #[doc(alias = "RENDER_START")]
    RenderStart = 5,

    #[doc(alias = "FRAME_RENDER_END")]
    #[doc(alias = "RENDER_END")]
    RenderEnd = 6,

    #[doc(alias = "FRAME_NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    #[doc(alias = "NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    FullFrameUpdateOnRemove = 7,
}

impl Frame {
    /// Is either `UpdateStart`, `UpdateEnd`, `PostDataStart`, or `PostDataEnd`.
    pub const fn is_net(&self) -> bool {
        matches!(
            &self,
            Frame::UpdateStart | Frame::UpdateEnd | Frame::PostDataStart | Frame::PostDataEnd
        )
    }

    /// Is either `RenderStart` or RenderEnd`.
    pub const fn is_render(&self) -> bool {
        matches!(*self, Frame::RenderStart | Frame::RenderEnd)
    }
}
