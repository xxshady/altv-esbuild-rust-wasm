use async_executor::{spawn_future, EXECUTOR_INSTANCE};
use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};
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

  #[wasm_bindgen]
  fn enable_altv_event(event_name: &str);

  #[wasm_bindgen]
  fn disable_altv_event(event_name: &str);
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

  let future = async {
    let player_name = crate::base_objects::scope::new_scope(|scope| {
      let player = base_objects::player::Player::get_by_id(scope, 123);
      // let's assume player with such id is valid on this tick
      let player = player.unwrap();

      // no need to hold reference to player instance if we only need data from it
      player.name()
    });
    wait(web_time::Duration::from_secs(1)).await;
    dbg!(player_name);
  };

  spawn_future(future);
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
