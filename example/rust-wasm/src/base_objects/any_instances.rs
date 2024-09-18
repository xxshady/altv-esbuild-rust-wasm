use crate::wasm_imports;

use super::{
  base_object_js_ref::BaseObjectJsRef,
  as_base_object_type::AsBaseObjectType,
  attached_to_scope::AttachedToScope,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    world_object::WorldObject,
  },
  handle::BaseObjectHandle,
  local_player::{local_player, LocalPlayer},
  player::{Player, ScopedPlayer},
};

pub enum AnyBaseObject {
  Player(Player),
  // TODO:
  // Vehicle(Vehicle),
}

#[derive(Debug)]
pub enum AnyPlayer<'scope> {
  Remote(ScopedPlayer<'scope>),
  Local(&'static LocalPlayer),
}

impl<'scope> BaseObjectJsRef for AnyPlayer<'scope> {
  fn js_ref(&self) -> &wasm_imports::BaseObject {
    match self {
      Self::Local(instance) => &instance.js_ref,
      Self::Remote(instance) => &instance.js_ref,
    }
  }
}
