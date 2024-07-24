use super::{instance::BaseObject, scope::Scope, scoped_instance::ScopedBaseObject};

/// Base object instance detached from [`scope`](super::scope::Scope),
/// "unscoped" means it's not known whether its still valid or not
/// so you can't do anything with this except [`attach`](Self::attach_to_scope) it again
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
pub struct UnscopedBaseObject<T> {
  instance: BaseObject<T>,
}

impl<T> UnscopedBaseObject<T> {
  pub(crate) fn new(instance: BaseObject<T>) -> Self {
    Self { instance }
  }

  pub fn attach_to_scope<'scope>(
    &self,
    scope: &'scope Scope,
  ) -> Option<ScopedBaseObject<'scope, T>> {
    todo!()
  }
}
