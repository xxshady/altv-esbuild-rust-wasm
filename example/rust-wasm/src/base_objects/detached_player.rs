use super::{
  player::{Player, ScopedPlayer},
  scope::Scope,
};

// TODO: add generic `DetachedBaseObject` or `UnscopedBaseObject` (see `ScopedBaseObject`)

/// Player instance "detached" from [`scope`](super::scope::Scope),
/// "detached" means it's not known whether its still valid or not
/// so you can't do anything with this except [`attach`](Self::attach_to_scope) it again
///
/// # Example
///
/// ```
/// altv::events::on_game_entity_create(|context| {
///   let altv::AnyEntity::Player(player) = context.entity {
///     return;
///   };
///   let detached_player = player.detach_from_scope();
///   
///   altv::set_timeout(|context| {
///     let Some(player) = detached_player.attach_to_scope(context) else {
///       return;
///     };
///     dbg!(player);
///   }, Duration::from_secs(1));
/// });
/// ```
pub struct DetachedPlayer {}

impl DetachedPlayer {
  pub fn attach_to_scope<'scope>(&self, scope: &'scope Scope) -> Option<ScopedPlayer<'scope>> {
    todo!()
  }
}
