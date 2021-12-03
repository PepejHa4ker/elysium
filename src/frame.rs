use crate::entity::Entity;

#[derive(Debug)]
pub enum Frame<'a> {
    RenderStart { local_player: Option<&'a Entity> },
}
