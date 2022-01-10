#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Frame(i32);

impl Frame {
    pub const START: Frame = Frame(0);
    pub const NET_UPDATE_START: Frame = Frame(1);
    pub const NET_UPDATE_POST_DATA_UPDATE_START: Frame = Frame(2);
    pub const NET_UPDATE_POST_DATA_UPDATE_END: Frame = Frame(3);
    pub const NET_UPDATE_END: Frame = Frame(4);
    pub const RENDER_START: Frame = Frame(5);
    pub const RENDER_END: Frame = Frame(6);
}
