#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Frame {
    #[doc(alias = "START")]
    Start = 0,

    #[doc(alias = "NET_UPDATE_START")]
    UpdateStart = 1,

    #[doc(alias = "NET_UPDATE_END")]
    UpdateEnd = 4,

    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_START")]
    PostDataStart = 2,

    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_END")]
    PostDataEnd = 3,

    #[doc(alias = "RENDER_START")]
    RenderStart = 5,

    #[doc(alias = "RENDER_END")]
    RenderEnd = 6,
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
