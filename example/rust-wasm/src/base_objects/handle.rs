use serde::{Deserialize, Serialize};

use crate::{id_provider::Id, wasm_imports};

use super::base_object_type::BaseObjectType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BaseObjectHandle {
  pub btype: BaseObjectType,
  pub id: u32,
  pub generation: Id,
}

impl BaseObjectHandle {
  pub fn as_js_ref(&self) -> wasm_imports::BaseObject {
    let Some(base_object_ref) = wasm_imports::get_base_object_ref(self.btype as u8, self.id) else {
      panic!("Expected valid base object: {self:?}");
    };
    base_object_ref
  }
}

pub trait BaseObjectSpecificHandle: Copy {
  fn to_base(&self) -> BaseObjectHandle;
}
