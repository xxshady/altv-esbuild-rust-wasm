use std::ops::Deref;

use super::{instance::BaseObject, scope::Scope};

// TODO: "scoped" and "unscoped" or "attached" and "detached"?
// TODO: add Deref<Target = BaseObject<T>> for this?
pub struct ScopedBaseObject<'scope, T> {
  scope: &'scope Scope,
  instance: BaseObject<T>,
}

impl<'scope, T> ScopedBaseObject<'scope, T> {
  pub(crate) fn new(scope: &'scope Scope, instance: BaseObject<T>) -> Self {
    Self { scope, instance }
  }
}
