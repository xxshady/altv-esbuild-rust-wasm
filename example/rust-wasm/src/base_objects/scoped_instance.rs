use std::ops::Deref;

use super::{instance::BaseObject, scope::Scope, unscoped_instance::UnscopedBaseObject};

// TODO: add Deref<Target = BaseObject<T>> for this?

/// Base object instance attached from [`scope`](super::scope::Scope),
/// "scoped" means it's attached to a scope and can only be used while that scope is alive.
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
pub struct ScopedBaseObject<'scope, T> {
  scope: &'scope Scope,
  instance: BaseObject<T>,
}

impl<'scope, T> ScopedBaseObject<'scope, T> {
  pub(crate) fn new(scope: &'scope Scope, instance: BaseObject<T>) -> Self {
    Self { scope, instance }
  }

  // TODO: change to &self and clone instance?
  pub fn detach_from_scope(self) -> UnscopedBaseObject<T> {
    UnscopedBaseObject::new(self.instance)
  }
}
