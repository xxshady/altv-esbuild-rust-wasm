use std::{fmt::Debug, ops::Deref};

use crate::base_objects::handle::GenericBaseObjectHandle;

use super::{
  as_base_object_type::AsBaseObjectType, handle::BaseObjectHandle, instance::BaseObject,
  scope::Scope,
};

/// Base object instance attached to a [`scope`](super::scope::Scope)
/// and can only be used while that scope is alive (in other words, *base object is owned by its scope*).
///
/// See also the [`handle method`](super::instance::BaseObject::handle) for a way to get an owned reference to base object,
/// which can be passed anywhere and re-attached to some scope.
///
/// # Example
///
/// ```
/// altv::events::on_game_entity_create(|context| {
///   let altv::AnyEntity::Player(player) = context.entity {
///     return;
///   };
///   let unscoped_player = player.detach_from_scope();
///   
///   altv::set_timeout(|context| {
///     let Some(player) = unscoped_player.attach_to_scope(context) else {
///       return;
///     };
///     dbg!(player);
///   }, Duration::from_secs(1));
/// });
/// ```
pub struct ScopedBaseObject<'scope, T: AsBaseObjectType> {
  _scope: &'scope dyn Scope,
  instance: BaseObject<T>,
}

impl<'scope, T: AsBaseObjectType> ScopedBaseObject<'scope, T> {
  pub(crate) fn new(scope: &'scope impl Scope, instance: BaseObject<T>) -> Self {
    Self {
      _scope: scope,
      instance,
    }
  }
}

impl<'scope, T: AsBaseObjectType> Debug for ScopedBaseObject<'scope, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let GenericBaseObjectHandle {
      id,
      btype,
      generation,
    } = self.instance.handle.as_base();
    write!(
      f,
      "ScopedBaseObject {{ id: {id}, type: {btype:?}, generation: {generation:?} }}"
    )
  }
}

impl<'scope, T: AsBaseObjectType> Deref for ScopedBaseObject<'scope, T> {
  type Target = BaseObject<T>;

  fn deref(&self) -> &Self::Target {
    &self.instance
  }
}
