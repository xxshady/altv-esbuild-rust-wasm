use std::any::Any;

use super::{
  any_instance::AnyBaseObject,
  handle::BaseObjectHandle,
  instance::{BaseObject},
  manager::Manager,
};

/// Scope where "borrowed" base objects (for example Player or Vehicle) are guaranteed to be valid
///
/// # Example
///
/// ```
/// // `context` implements Scope
/// altv::set_timeout(|context| {
///   // `streamed_in` needs scope to guarantee that
///   // returned players will not be used after destroy
///   let players = altv::Player::streamed_in(context);
///
///   altv::spawn_local(async {
///     dbg!(players); // error, `context` is dead at this moment
///   });
/// });
/// ```
#[derive(Default)]
pub struct Scope {
  base_objects: Vec<AnyBaseObject>,
}

impl Scope {
  pub fn attach_base_object<'scope, T>(
    &'scope mut self,
    base_object: AnyBaseObject,
  ) -> &'scope BaseObject<T> {
    self.base_objects.push(base_object);
    self.base_objects.last().unwrap().into()
  }
}

pub fn new_scope<R>(use_scope: impl for<'scope> FnOnce(&'scope Scope) -> R) -> R {
  use_scope(&Scope::default())
}
