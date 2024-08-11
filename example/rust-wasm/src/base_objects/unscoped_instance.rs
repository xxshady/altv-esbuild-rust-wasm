use super::{
  handle::BaseObjectSpecificHandle, instance::BaseObject, manager::MANAGER_INSTANCE, scope::Scope,
  scoped_instance::ScopedBaseObject,
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
pub struct UnscopedBaseObject<H: BaseObjectSpecificHandle> {
  instance: BaseObject<H>,
}

impl<H: BaseObjectSpecificHandle> UnscopedBaseObject<H> {
  pub(crate) fn new(instance: BaseObject<H>) -> Self {
    Self { instance }
  }

  pub fn scope<'scope>(&self, scope: &'scope impl Scope) -> Option<ScopedBaseObject<'scope, H>> {
    let valid = MANAGER_INSTANCE
      .with_borrow(|manager| manager.is_handle_valid(&self.instance.handle.to_base()));

    if valid {
      Some(scope.attach_base_object(self.instance.clone()))
    } else {
      None
    }
  }
}
