use std::cell::RefCell;

use async_executor::EXECUTOR_INSTANCE;
use timers::{TIMER_MANAGER_INSTANCE, TIMER_SCHEDULE_INSTANCE};
use wasm_bindgen::prelude::*;

mod altv_events;
mod script_events;
mod async_executor;
mod logging;
mod wasm_imports;
mod id_provider;
mod panic_handler;
use logging::log_info;
mod timers;
mod wait;
mod base_objects;

#[wasm_bindgen]
pub fn main() {
  panic_handler::init();

  log_info!("start");

  base_objects::manager::init();
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
  // serverside
  // let base_object = wasm_imports::Vehicle::new(0x3404691C, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
  // log_info!("dimension: {}", base_object.dimension());
  // base_object.set_dimension(123);
  // log_info!("dimension: {}", base_object.dimension());
  // log_info!("model: {}", base_object.model() == 0x3404691C);
  // log_info!("color: {}", base_object.primaryColor());

  // // clientside
  // mod altv {
  //   use super::*;
  //   pub use base_objects::{scope::new_scope, player::Player};
  //   pub use timers::set_timeout;
  // }

  // let unscoped_player = altv::new_scope(|scope| {
  //   let vehicle = altv::Player::get_by_handle(scope, base_object.handle()).unwrap();
  //   log_info!("veh: {vehicle:?}");

  //   vehicle.unscope()
  // });

  // base_object.destroy();

  // altv::set_timeout(
  //   move |scope| {
  //     log_info!("1");
  //     let try_scope_vehicle = unscoped_player.scope(scope);
  //     log_info!("2");
  //     log_info!("try_scope_vehicle: {try_scope_vehicle:?}");
  //   },
  //   Duration::from_secs(1),
  // );
}
