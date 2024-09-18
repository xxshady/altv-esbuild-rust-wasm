use std::{
  cell::OnceCell,
  sync::{OnceLock, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::wasm_imports;

use super::{
  as_base_object_type::AsBaseObjectType,
  base_object_type::BaseObjectType,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    world_object::WorldObject,
  },
  attached_to_scope::AttachedToScope,
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, GenericBaseObjectHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub type LocalPlayer = BaseObject<LocalPlayerType>;
pub type LocalPlayerHandle = BaseObjectHandle<LocalPlayerType>;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct LocalPlayerType;

impl AsBaseObjectType for LocalPlayerType {
  fn as_base_object_type() -> BaseObjectType {
    BaseObjectType::LocalPlayer
  }
}

impl class_traits::local_player::LocalPlayer for LocalPlayer {}
impl WorldObject for LocalPlayer {}
impl Entity for LocalPlayer {}

// TODO: is it needed?
// impl SyncedEntity for LocalPlayer {}

// doesn't matter here anyway because we don't have multi threading in WASM (yet?)
unsafe impl Send for LocalPlayer {}
unsafe impl Sync for LocalPlayer {}

pub fn local_player() -> &'static LocalPlayer {
  static INSTANCE: OnceLock<LocalPlayer> = OnceLock::new();

  INSTANCE.get_or_init(|| {
    let handle = LocalPlayerHandle::new(1, 1);
    let instance = LocalPlayer::new(handle, wasm_imports::get_local_player());

    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      manager.on_create(handle.as_generic());
    });

    instance
  })
}

// pub(crate) fn local_player_scope() -> &'static dyn Scope {
//   struct LocalPlayerScope;
//   impl Scope for LocalPlayerScope {}

//   static INSTANCE: OnceLock<LocalPlayerScope> = OnceLock::new();
//   INSTANCE.get_or_init(|| LocalPlayerScope)
// }
