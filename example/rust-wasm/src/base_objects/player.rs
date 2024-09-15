use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;

use crate::{timers::set_timeout, wasm_imports};

use super::{
  base_object_type::BaseObjectType,
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, BaseObjectSpecificHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::{new_scope, Scope},
  scoped_instance::ScopedBaseObject,
};

pub type Player = BaseObject<PlayerHandle>;
pub type ScopedPlayer<'scope> = ScopedBaseObject<'scope, PlayerHandle>;

impl Player {
  pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> Vec<ScopedPlayer<'scope>> {
    let players = wasm_imports::get_streamed_in_players();
    let players: Vec<PlayerHandle> = from_value(players).unwrap();

    players
      .into_iter()
      .map(|handle| handle.attach_to(scope).unwrap())
      .collect()
  }

  pub fn name(&self) -> String {
    // TEST
    wasm_imports::get_player_name(&self.js_ref)
  }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PlayerHandle {
  id: BaseObjectId,
  generation: BaseObjectGeneration,
}

impl BaseObjectSpecificHandle for PlayerHandle {
  fn to_base(self) -> BaseObjectHandle {
    BaseObjectHandle {
      btype: BaseObjectType::Player,
      id: self.id,
      generation: self.generation,
    }
  }

  fn attach_to<'scope>(self, scope: &'scope impl Scope) -> Option<ScopedPlayer<'scope>> {
    MANAGER_INSTANCE.with_borrow(|manager| {
      let player = BaseObject::new_by_handle(manager, self)?;
      Some(scope.attach_base_object(player))
    })
  }
}

fn _test_player() {
  new_scope(|scope| {
    let players = Player::streamed_in(scope);
    let [player] = &players[..] else {
      return;
    };
    let player_handle = player.handle();

    set_timeout(
      move |scope| {
        let Some(_player) = player_handle.attach_to(scope) else {
          return;
        };
      },
      Duration::from_secs(1),
    );
  });
}
