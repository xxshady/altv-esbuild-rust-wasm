use super::{
  any_instance::{AnyBaseObject},
  base_object_type::BaseObjectType,
  detached_player::DetachedPlayer,
  handle::BaseObjectHandle,
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub struct PlayerType;

pub type Player = BaseObject<PlayerType>;
pub type ScopedPlayer<'scope> = ScopedBaseObject<'scope, PlayerType>;

impl Player {
  pub fn get_by_id<'scope>(scope: &'scope Scope, id: u32) -> Option<ScopedPlayer<'scope>> {
    MANAGER_INSTANCE.with_borrow(|manager| {
      let player = BaseObject::new_by_handle(
        manager,
        BaseObjectHandle {
          id,
          btype: BaseObjectType::PLAYER,
        },
      )?;
      Some(scope.attach_base_object(player))
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
