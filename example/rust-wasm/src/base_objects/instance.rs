use std::marker::PhantomData;

use crate::wasm_imports::BaseObject as JsBaseObjectRef;

use super::{handle::BaseObjectHandle, manager::Manager};

#[derive(Clone)]
pub struct BaseObject<T: Clone> {
  pub(crate) handle: BaseObjectHandle,
  pub(crate) _type: PhantomData<T>,
  js_ref: JsBaseObjectRef,
}

impl<T: Clone> BaseObject<T> {
  pub fn id(&self) -> u32 {
    self.js_ref.id()
  }

  pub fn new_by_handle(manager: &Manager, handle: BaseObjectHandle) -> Option<Self> {
    let valid = manager.is_handle_valid(&handle);
    if valid {
      let js_ref = handle.as_js_ref();
      Some(Self {
        handle,
        js_ref,
        _type: PhantomData,
      })
    } else {
      None
    }
  }
}
