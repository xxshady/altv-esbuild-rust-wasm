use crate::base_objects::base_object_js_ref::BaseObjectJsRef;

pub trait LocalPlayer: BaseObjectJsRef {
  fn current_ammo(&self) -> u16 {
    self.js_ref().current_ammo()
  }
}
