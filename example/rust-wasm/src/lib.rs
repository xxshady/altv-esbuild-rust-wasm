use async_executor::EXECUTOR_INSTANCE;
use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
use timers::{TIMER_MANAGER_INSTANCE, TIMER_SCHEDULE_INSTANCE};
use wasm_bindgen::prelude::*;

mod altv_events;
mod async_executor;
mod logging;
mod timers;
mod wait;

#[wasm_bindgen(js_namespace = altv_imports)]
extern "C" {
  #[wasm_bindgen]
  fn log_info(data: &str);

  #[wasm_bindgen]
  fn log_warn(data: &str);

  #[wasm_bindgen]
  fn enable_altv_event(event_name: &str);

  #[wasm_bindgen]
  fn disable_altv_event(event_name: &str);
}

#[wasm_bindgen]
pub fn main() {
  console_error_panic_hook::set_once();
  log_info("start");
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
