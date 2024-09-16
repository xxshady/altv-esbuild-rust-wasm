use std::marker::PhantomData;

use crate::wasm_imports::BaseObject as JsBaseObjectRef;
use super::{
  as_base_object_type::AsBaseObjectType,
  handle::{GenericBaseObjectHandle, BaseObjectHandle},
  manager::Manager,
};

#[derive(Clone)]
pub struct BaseObject<T: AsBaseObjectType> {
  pub(crate) handle: BaseObjectHandle<T>,
  pub(crate) js_ref: JsBaseObjectRef,
}

impl<T: AsBaseObjectType> BaseObject<T> {
  pub(crate) fn new_by_handle(manager: &Manager, handle: BaseObjectHandle<T>) -> Option<Self> {
    let base_handle = handle.as_base();
    let valid = manager.is_handle_valid(&base_handle);
    if valid {
      let js_ref = base_handle.as_js_ref();
      Some(Self { handle, js_ref })
    } else {
      None
    }
  }

  /// See [`BaseObjectSpecificHandle`](super::handle::BaseObjectSpecificHandle).
  pub fn handle(&self) -> BaseObjectHandle<T> {
    self.handle
  }
}
