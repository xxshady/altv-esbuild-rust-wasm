use std::fmt::Debug;

use super::{instance::BaseObject, scope::Scope, unscoped_instance::UnscopedBaseObject};

// TODO: add Deref<Target = BaseObject<T>> for this?

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
pub struct ScopedBaseObject<'scope, T: Clone> {
  _scope: &'scope Scope,
  instance: BaseObject<T>,
}

impl<'scope, T: Clone> ScopedBaseObject<'scope, T> {
  pub(crate) fn new(scope: &'scope Scope, instance: BaseObject<T>) -> Self {
    Self {
      _scope: scope,
      instance,
    }
  }

  pub fn unscope(&self) -> UnscopedBaseObject<T> {
    UnscopedBaseObject::new(self.instance.clone())
  }
}

impl<'scope, T: Clone> Debug for ScopedBaseObject<'scope, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let id = self.instance.id();
    let btype = self.instance._type;
    write!(f, "ScopedBaseObject {{ id: {id}, type: {btype:?} }}")
  }
}
