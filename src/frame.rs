use crate::entity::Entity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Frame {
    Undefined = -1,
    Start = 0,
    NetUpdateStart = 1,
    NetUpdatePostDataUpdateStart = 2,
    NetUpdatePostDataUpdateEnd = 3,
    NetUpdateEnd = 4,
    RenderStart = 6,
    RenderEnd = 7,
}
