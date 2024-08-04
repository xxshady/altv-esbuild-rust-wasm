use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = altv_imports)]
extern "C" {
  #[wasm_bindgen]
  pub fn log_info(data: &str);

  #[wasm_bindgen]
  pub fn log_warn(data: &str);

  // TODO: dont send event as string for better performance
  #[wasm_bindgen]
  pub fn enable_altv_event(event_name: &str);
  #[wasm_bindgen]
  pub fn disable_altv_event(event_name: &str);

  #[wasm_bindgen]
  pub fn get_base_object_ref(btype: u8, id: u32) -> Option<BaseObject>;

  #[derive(Clone)]
  pub type BaseObject;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &BaseObject) -> u32;

  #[wasm_bindgen(method)]
  pub fn destroy(this: &BaseObject);

  #[wasm_bindgen(extends = BaseObject)]
  pub type WorldObject;

  #[wasm_bindgen(method, getter)]
  pub fn dimension(this: &WorldObject) -> i32;
  #[wasm_bindgen(method, setter)]
  pub fn set_dimension(this: &WorldObject, value: i32);

  #[wasm_bindgen(extends = WorldObject)]
  pub type Entity;

  #[wasm_bindgen(method, getter)]
  pub fn model(this: &Entity) -> u32;

  #[wasm_bindgen(extends = Entity)]
  pub type Vehicle;

  #[wasm_bindgen(constructor)]
  pub fn new(
    model: u32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
  ) -> Vehicle;

  #[wasm_bindgen(method, getter)]
  pub fn primaryColor(this: &Vehicle) -> u8;
}
