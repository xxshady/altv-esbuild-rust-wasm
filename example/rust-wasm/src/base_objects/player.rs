use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use crate::wasm_imports;

use super::{
  base_object_type::BaseObjectType,
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectSpecificHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::{new_scope, Scope},
  scoped_instance::ScopedBaseObject,
};

pub type Player = BaseObject<PlayerHandle>;
pub type ScopedPlayer<'scope> = ScopedBaseObject<'scope, PlayerHandle>;

impl Player {
  pub fn get_by_handle<'scope>(
    scope: &'scope impl Scope,
    handle: impl Into<PlayerHandle>,
  ) -> Option<ScopedPlayer<'scope>> {
    let handle = handle.into();

    MANAGER_INSTANCE.with_borrow(|manager| {
      let player = BaseObject::new_by_handle(manager, handle)?;
      Some(scope.attach_base_object(player))
    })
  }

  pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> Vec<ScopedPlayer<'scope>> {
    let players = wasm_imports::get_streamed_in_players();
    let players: Vec<PlayerHandle> = from_value(players).unwrap();

    players
      .into_iter()
      .map(|handle| Self::get_by_handle(scope, handle).unwrap())
      .collect()
  }

  pub fn name(&self) -> String {
    todo!()
  }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlayerHandle {
  id: u32,
  generation: BaseObjectGeneration,
}

impl BaseObjectSpecificHandle for PlayerHandle {
  fn to_base(&self) -> BaseObjectHandle {
    BaseObjectHandle {
      btype: BaseObjectType::Player,
      id: self.id,
      generation: self.generation,
    }
  }
}

impl From<&BaseObjectHandle> for PlayerHandle {
  fn from(value: &BaseObjectHandle) -> Self {
    Self {
      id: value.id,
      generation: value.generation,
    }
  }
}

fn _test_player() {
  new_scope(|scope| {
    let players = Player::streamed_in(scope);
    let [player] = players.as_slice() else {
      return;
    };

    let _ = Player::get_by_handle(scope, player.handle());
  });
}
