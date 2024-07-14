use super::{
  base_object_type::BaseObjectType,
  detached_player::DetachedPlayer,
  handle::BaseObjectHandle,
  instance::{BaseObject},
  manager::MANAGER_INSTANCE,
  scope::Scope,
};

struct PlayerType;

pub type Player = BaseObject<PlayerType>;

impl Player {
  pub fn get_by_id<'scope>(scope: &'scope Scope, id: u32) -> Option<&'scope Player> {
    MANAGER_INSTANCE.with_borrow(|manager| {
      scope.attach_base_object(
        manager,
        BaseObjectHandle {
          id,
          btype: BaseObjectType::PLAYER,
        },
      )
    })
  }

  // pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> &'scope [Player] {}

  pub fn detach_from_scope(&self) -> DetachedPlayer {
    DetachedPlayer {}
  }

  pub fn name(&self) -> String {
    todo!()
  }
}
