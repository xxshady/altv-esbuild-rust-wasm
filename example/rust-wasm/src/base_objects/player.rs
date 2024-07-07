use super::{detached_player::DetachedPlayer, scope::Scope};

#[derive(Debug)]
pub struct Player {}

impl Player {
  pub fn get_by_id<'scope>(scope: &'scope impl Scope, id: u32) -> Option<&'scope Player> {
    Some(&Player {})
  }

  pub fn detach_from_scope(&self) -> DetachedPlayer {
    DetachedPlayer {}
  }

  pub fn name(&self) -> String {
    todo!()
  }
}
