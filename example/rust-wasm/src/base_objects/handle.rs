use serde::{Deserialize, Serialize};

use super::base_object_type::BaseObjectType;

// TODO: generation id?
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct BaseObjectHandle {
  pub(crate) id: u32,
  pub(crate) btype: BaseObjectType,
}

impl BaseObjectHandle {
  pub(crate) fn as_js_ref(&self) -> crate::BaseObject {
    let Some(base_object_ref) = crate::get_base_object_ref(self.id, self.btype as u8) else {
      panic!("Failed to get ref to base object: {self:?}");
    };
    base_object_ref
  }
}
