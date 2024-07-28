use async_executor::{spawn_future, EXECUTOR_INSTANCE};
use base_objects::{base_object_type::BaseObjectType, manager::MANAGER_INSTANCE};
use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
use serde::{Deserialize, Serialize};
use timers::{set_timeout, TIMER_MANAGER_INSTANCE, TIMER_SCHEDULE_INSTANCE};
use wait::wait;
use wasm_bindgen::prelude::*;

mod altv_events;
mod async_executor;
mod logging;
mod timers;
mod wait;
pub mod base_objects;

#[wasm_bindgen(js_namespace = altv_imports)]
extern "C" {
  #[wasm_bindgen]
  fn log_info(data: &str);

  #[wasm_bindgen]
  fn log_warn(data: &str);

  // TODO: dont send event as string for better performance
  #[wasm_bindgen]
  fn enable_altv_event(event_name: &str);
  #[wasm_bindgen]
  fn disable_altv_event(event_name: &str);

  #[wasm_bindgen]
  fn get_base_object_ref(id: u32, btype: u8) -> Option<BaseObject>;

  #[derive(Clone)]
  type BaseObject;

  #[wasm_bindgen(method, getter)]
  fn id(this: &BaseObject) -> u32;

  #[wasm_bindgen(extends = BaseObject)]
  type WorldObject;

  #[wasm_bindgen(method, getter)]
  fn dimension(this: &WorldObject) -> i32;
  #[wasm_bindgen(method, setter)]
  fn set_dimension(this: &WorldObject, value: i32);

  #[wasm_bindgen(extends = WorldObject)]
  type Entity;

  #[wasm_bindgen(method, getter)]
  fn model(this: &Entity) -> u32;

  #[wasm_bindgen(extends = Entity)]
  type Vehicle;

  #[wasm_bindgen(constructor)]
  fn new(
    model: u32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
  ) -> Vehicle;

  #[wasm_bindgen(method, getter)]
  fn primaryColor(this: &Vehicle) -> u8;

}

#[wasm_bindgen]
pub fn main() {
  console_error_panic_hook::set_once();
  log_info("start");

  // let mut future = None;

  // set_timeout(
  //   move |context| {
  //     let player = base_objects::player::Player::get_by_id(context, 123);
  //     let player = player.unwrap();

  //     future = Some(async {
  //       log_info(&format!("player: {player:?}"));
  //     });
  //   },
  //   web_time::Duration::from_secs(1),
  // );

  // let future = async {
  //   let player_name = crate::base_objects::scope::new_scope(|scope| {
  //     let player = base_objects::player::Player::get_by_id(scope, 123);
  //     // let's assume player with such id is valid on this tick
  //     let player = player.unwrap();

  //     // no need to hold reference to player instance if we only need data from it
  //     player.name()
  //   });
  //   wait(web_time::Duration::from_secs(1)).await;
  //   dbg!(player_name);
  // };

  // spawn_future(future);
}

#[wasm_bindgen]
pub fn on_every_tick() {
  EXECUTOR_INSTANCE.with_borrow_mut(|executor| {
    executor.run();
  });
  TIMER_MANAGER_INSTANCE.with_borrow_mut(|timers| {
    TIMER_SCHEDULE_INSTANCE.with(|schedule| timers.process_timers(schedule.borrow_mut()));
  });
}

#[wasm_bindgen]
pub fn test_base_object() {
  let base_object = Vehicle::new(0x3404691C, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  log_info(&format!("dimension: {}", base_object.dimension()));
  base_object.set_dimension(123);
  log_info(&format!("dimension: {}", base_object.dimension()));
  log_info(&format!("model: {}", base_object.model() == 0x3404691C));
  log_info(&format!("color: {}", base_object.primaryColor()));
}
