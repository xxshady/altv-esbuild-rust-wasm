use serde::{Deserialize, Serialize};

use crate::{id_provider::Id, wasm_imports};

use super::{
  base_object_type::{rust_to_sdk_base_object_type, sdk_to_rust_base_object_type, BaseObjectType},
  sdk_base_object_type::SdkBaseObjectType,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BaseObjectGeneration(Id);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BaseObjectHandle {
  pub btype: BaseObjectType,
  pub id: u32,
  pub generation: BaseObjectGeneration,
}

impl BaseObjectHandle {
  pub fn as_js_ref(&self) -> wasm_imports::BaseObject {
    let (sdk_type, is_remote) = rust_to_sdk_base_object_type(self.btype);
    let Some(base_object_ref) =
      wasm_imports::get_base_object_ref(sdk_type as u8, is_remote, self.id)
    else {
      panic!("Expected valid base object: {self:?}");
    };
    base_object_ref
  }
}

pub trait BaseObjectSpecificHandle: Copy {
  fn to_base(&self) -> BaseObjectHandle;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RawBaseObjectHandle {
  pub sdk_type: SdkBaseObjectType,
  pub is_remote: bool,
  pub id: u32,
  pub generation: BaseObjectGeneration,
}

impl RawBaseObjectHandle {
  pub fn as_handle(&self) -> BaseObjectHandle {
    BaseObjectHandle {
      btype: sdk_to_rust_base_object_type(self.sdk_type, self.is_remote),
      id: self.id,
      generation: self.generation,
    }
  }
}
