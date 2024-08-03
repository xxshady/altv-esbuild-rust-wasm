use crate::log_info;

use super::{
  instance::BaseObject, manager::MANAGER_INSTANCE, scope::Scope, scoped_instance::ScopedBaseObject,
};

/// Detached base object instance, it's not known whether its still valid or not
/// so you can't do anything with it except [`scope`](Self::scope) it again
///
/// The opposite of [`ScopedInstance`](super::scoped_instance::ScopedBaseObject).
///
/// # Example
///
/// ```
/// altv::events::on_game_entity_create(|context| {
///   let altv::AnyEntity::Player(player) = context.entity {
///     return;
///   };
///   let unscoped_player = player.unscope();
///   
///   altv::set_timeout(|context| {
///     let Some(player) = unscoped_player.scope(context) else {
///       return;
///     };
///     dbg!(player);
///   }, Duration::from_secs(1));
/// });
/// ```
pub struct UnscopedBaseObject<T: Clone> {
  instance: BaseObject<T>,
}

impl<T: Clone> UnscopedBaseObject<T> {
  pub(crate) fn new(instance: BaseObject<T>) -> Self {
    Self { instance }
  }

  pub fn scope<'scope>(&self, scope: &'scope Scope) -> Option<ScopedBaseObject<'scope, T>> {
    let valid =
      MANAGER_INSTANCE.with_borrow(|manager| manager.is_handle_valid(&self.instance.handle));

    if valid {
      Some(scope.attach_base_object(self.instance.clone()))
    } else {
      None
    }
  }
}
