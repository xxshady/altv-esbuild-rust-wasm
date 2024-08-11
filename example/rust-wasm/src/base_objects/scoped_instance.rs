use std::{fmt::Debug, ops::Deref};

use crate::base_objects::handle::BaseObjectHandle;

use super::{
  handle::BaseObjectSpecificHandle, instance::BaseObject, scope::Scope,
  unscoped_instance::UnscopedBaseObject,
};

/// Base object instance attached to a [`scope`](super::scope::Scope)
/// and can only be used while that scope is alive (in other words, *base object is owned by its scope*).
///
/// The opposite of [`UnscopedInstance`](super::unscoped_instance::UnscopedBaseObject).
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
pub struct ScopedBaseObject<'scope, H: BaseObjectSpecificHandle> {
  _scope: &'scope dyn Scope,
  instance: BaseObject<H>,
}

impl<'scope, H: BaseObjectSpecificHandle> ScopedBaseObject<'scope, H> {
  pub(crate) fn new(scope: &'scope impl Scope, instance: BaseObject<H>) -> Self {
    Self {
      _scope: scope,
      instance,
    }
  }

  pub fn unscope(&self) -> UnscopedBaseObject<H> {
    UnscopedBaseObject::new(self.instance.clone())
  }
}

impl<'scope, H: BaseObjectSpecificHandle> Debug for ScopedBaseObject<'scope, H> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let BaseObjectHandle {
      id,
      btype,
      generation,
    } = self.instance.handle.to_base();
    write!(
      f,
      "ScopedBaseObject {{ id: {id}, type: {btype:?}, generation: {generation:?} }}"
    )
  }
}

impl<'scope, H: BaseObjectSpecificHandle> Deref for ScopedBaseObject<'scope, H> {
  type Target = BaseObject<H>;

  fn deref(&self) -> &Self::Target {
    &self.instance
  }
}
