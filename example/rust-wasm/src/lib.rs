use std::{
    cell::RefCell,
    future::{poll_fn, Future},
    task::{Poll, Waker},
    time::Duration,
};

use async_executor::{spawn_future, EXECUTOR_INSTANCE};
use timers::{set_timeout, TIMER_MANAGER_INSTANCE, TIMER_SCHEDULE_INSTANCE};
use wasm_bindgen::prelude::*;
use web_time::SystemTime;

mod async_executor;
mod timers;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = altv_imports)]
    fn log(string: &str);
}

#[wasm_bindgen]
pub fn call_rust_wasm() {
    log(&format!("Hello, alt:V! line: {}", line!()));

    spawn_future(async {
        start_loop(|tick| {
            let players = tick.all_players();
            dbg!(players);

            let player = tick.get_player_by_id(1);

            // not possible:
            // spawn_future(async {
            //     dbg!(players);
            //     dbg!(player);
            // });

            if let Some(ev) = tick.on_server_event() {
                match ev.name.as_str() {
                    "my-event" => {
                        // ...
                    }
                    _ => {}
                }
            }
        })
        .await;
    });

    #[allow(dead_code)]
    fn main(main_context: &MainContext) {
        let _players: &[Player] = main_context.all_players();

        every_tick(|tick: &EveryTickContext| {
            // cannot be called here
            // main_context.all_players();

            let _players: &[Player] = tick.all_players();
            let player: Option<&Player> = tick.get_player_by_id(1);

            // not possible:
            // spawn_future(async {
            //     dbg!(_players);
            //     dbg!(player);
            // });

            let Some(player) = player else {
                return;
            };
            let maybe = player.get_maybe();

            spawn_future(async {
                let Some(player) = maybe.into_player() else {
                    return;
                };
                dbg!(player);
            });
        });

        let mut _players: &[Player] = &[];
        on_server("some_event", |ctx: &OnServerEventContext| {
            let _data: &Vec<u8> = &ctx.data;

            let _players: &[Player] = ctx.all_players();

            // wont work
            // _players = players;
        });

        on_game_entity_create(|ctx: &GameEntityCreateContext| {
            let _entity: &AnyEntity = &ctx.entity;

            let _players: &[Player] = ctx.all_players();
        });
    }
}

async fn start_loop(every_tick: impl Fn(&EveryTickContext)) {
    loop {
        every_tick(&EveryTickContext {});
        wait(Duration::ZERO).await;
    }
}

// needs to be called from JS in every tick (a.k.a setInterval with 0 delay)
#[wasm_bindgen]
pub fn on_every_tick() {
    EXECUTOR_INSTANCE.with_borrow_mut(|executor| {
        executor.run();
    });
    TIMER_MANAGER_INSTANCE.with_borrow_mut(|timers| {
        TIMER_SCHEDULE_INSTANCE.with(|schedule| {
            timers.process_timers(schedule.borrow_mut());
        })
    })
}

pub fn wait(duration: Duration) -> impl Future {
    let dest = SystemTime::now() + duration;
    let mut timer_was_set = false;

    poll_fn(move |cx| {
        if SystemTime::now() >= dest {
            return Poll::Ready(());
        }
        if timer_was_set {
            return Poll::Pending;
        }
        timer_was_set = true;

        let waker = cx.waker().clone();
        // see my timers gist
        set_timeout(
            Box::new(|| {
                waker.wake();
            }),
            duration,
        );

        Poll::Pending
    })
}

#[derive(Debug)]
struct EveryTickContext {}

impl EveryTickContext {
    fn on_server_event(&self) -> Option<ServerEvent> {
        todo!()
    }
}

#[derive(Debug)]
struct Player {}

impl Player {
    fn get_maybe(&self) -> MaybePlayer {
        MaybePlayer { ptr: todo!() }
    }
}

struct MaybePlayer {
    ptr: usize,
}

impl MaybePlayer {
    /// Returns player if it exists
    fn into_player(self) -> Option<Player> {
        todo!()
    }
}

struct ServerEvent {
    name: String,
}

fn every_tick(_callback: impl FnMut(&EveryTickContext) + 'static) {
    todo!()
}

struct OnServerEventContext {
    data: Vec<u8>,
}

fn on_server(_event_name: &str, _handler: impl FnMut(&OnServerEventContext) + 'static) {
    todo!()
}

trait ThisTickApi {
    fn all_players(&self) -> &[Player] {
        todo!()
    }

    fn get_player_by_id(&self, _player_id: u32) -> Option<&Player> {
        todo!()
    }
}

impl ThisTickApi for EveryTickContext {}
impl ThisTickApi for OnServerEventContext {}
impl ThisTickApi for GameEntityCreateContext {}
impl ThisTickApi for MainContext {}

enum AnyEntity {}

struct GameEntityCreateContext {
    entity: AnyEntity,
}

fn on_game_entity_create(_handler: impl FnMut(&GameEntityCreateContext) + 'static) {
    todo!()
}

struct MainContext {}
