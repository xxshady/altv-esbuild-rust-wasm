use std::marker::PhantomData;

use crate::wasm_imports::BaseObject as JsBaseObjectRef;
use super::{
  handle::{BaseObjectHandle, BaseObjectSpecificHandle},
  manager::Manager,
};

#[derive(Clone)]
pub struct BaseObject<H: BaseObjectSpecificHandle> {
  pub(crate) handle: H,
  js_ref: JsBaseObjectRef,
}

impl<H: BaseObjectSpecificHandle> BaseObject<H> {
  pub fn new_by_handle(manager: &Manager, handle: H) -> Option<Self> {
    let base_handle = handle.to_base();
    let valid = manager.is_handle_valid(&base_handle);
    if valid {
      let js_ref = base_handle.as_js_ref();
      Some(Self { handle, js_ref })
    } else {
      None
    }
  }

  pub fn handle(&self) -> H {
    self.handle
  }
}
