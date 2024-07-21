use std::marker::PhantomData;

use crate::BaseObject as JsBaseObjectRef;

use super::{any_instance::AnyBaseObject, handle::BaseObjectHandle, manager::Manager};

pub struct BaseObject<T> {
  pub(crate) handle: BaseObjectHandle,
  _type: PhantomData<T>,
  js_ref: JsBaseObjectRef,
}

impl<T> BaseObject<T> {
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
