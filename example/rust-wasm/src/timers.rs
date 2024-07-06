use std::{
  cell::{RefCell, RefMut},
  fmt::Debug,
};

use web_time::{Duration, SystemTime};

pub type TimerId = u64;
pub type TimerCallback = dyn FnMut() + 'static;

thread_local! {
  pub(crate) static TIMER_SCHEDULE_INSTANCE: RefCell<ScheduleState> = Default::default();
  pub(crate) static TIMER_MANAGER_INSTANCE: RefCell<TimerManager> = Default::default();
}

struct TimerData {
  callback: Box<TimerCallback>,
  next_call_time: SystemTime,
  millis: u64,
  once: bool,
  id: TimerId,
}

// derive(Debug) didn't work because of `callback: Box<TimerCallback>`
impl Debug for TimerData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "TimerData {{ {} }}", self.id)
  }
}

#[derive(Debug, Default)]
pub struct ScheduleState {
  id: TimerId,
  timers: Vec<TimerData>,
  to_be_destroyed: Vec<TimerId>,
}

impl ScheduleState {
  pub fn create(&mut self, callback: Box<TimerCallback>, millis: u64, once: bool) -> TimerId {
    let id = {
      self.id += 1;
      self.id
    };

    let next_call_time = SystemTime::now() + Duration::from_millis(millis);

    self.timers.push(TimerData {
      callback,
      next_call_time,
      millis,
      once,
      id,
    });

    id
  }

  pub fn add_to_be_destroyed(&mut self, id: TimerId) {
    self.to_be_destroyed.push(id);
  }
}

#[derive(Debug, Default)]
pub struct TimerManager {
  timers: Vec<TimerData>,
}

impl TimerManager {
  pub fn process_timers(&mut self, mut schedule: RefMut<ScheduleState>) {
    self.timers.append(&mut schedule.timers);

    if !schedule.to_be_destroyed.is_empty() {
      for idx in self.get_to_be_destroyed(&schedule) {
        self.timers.swap_remove(idx);
      }
      schedule.to_be_destroyed.clear();
    }

    drop(schedule); // unborrow ScheduleState

    let mut indexes_to_remove: Vec<usize> = vec![];
    let now = SystemTime::now();

    for (idx, timer) in self.timers.iter_mut().enumerate().rev() {
      if now >= timer.next_call_time {
        (timer.callback)();

        if timer.once {
          indexes_to_remove.push(idx);
          continue;
        }
        timer.next_call_time = SystemTime::now() + Duration::from_millis(timer.millis);
      }
    }

    for idx in indexes_to_remove {
      self.timers.swap_remove(idx);
    }
  }

  fn get_to_be_destroyed(&self, schedule: &RefMut<ScheduleState>) -> Vec<usize> {
    let mut indexes: Vec<usize> = schedule
      .to_be_destroyed
      .iter()
      .filter_map(|id| {
        let timer = self
          .timers
          .iter()
          .enumerate()
          .find(|(_, t)| t.id == *id)
          .map(|(idx, _)| idx);

        timer.unwrap();

        // if timer.is_none() {
        //     logger::error!(
        //         "Failed to destroy timer with id: {id} (it was probably already removed)"
        //     );
        // } else {
        //     logger::debug!("destroying timer with id: {id}");
        // }
        timer
      })
      .collect();
    indexes.sort_unstable_by(|a, b| b.cmp(a));
    indexes
  }
}

pub fn create_timer(callback: Box<dyn FnMut() + 'static>, millis: u64, once: bool) -> Timer {
  let id = TIMER_SCHEDULE_INSTANCE.with_borrow_mut(|t| t.create(callback, millis, once));
  Timer::new(id)
}

pub fn remove_timer(id: TimerId) {
  TIMER_SCHEDULE_INSTANCE.with_borrow_mut(|t| {
    t.add_to_be_destroyed(id);
  });
}

#[derive(Debug)]
pub struct Timer {
  id: Option<TimerId>,
}

impl Timer {
  pub(crate) fn new(id: TimerId) -> Self {
    Self { id: Some(id) }
  }

  pub fn destroy(&mut self) {
    let Some(id) = self.id else {
      panic!("Already destroyed")
    };

    remove_timer(id);
  }

  pub fn id(&self) -> Option<TimerId> {
    self.id
  }
}

pub fn set_timeout(callback: impl FnOnce() + 'static, duration: Duration) -> Timer {
  let mut callback = Some(callback);
  create_timer(
    Box::new(move || (callback.take().unwrap())()),
    duration.as_millis() as u64, // TODO: use Duration
    true,
  )
}

pub fn set_interval(mut callback: impl FnMut() + 'static, duration: Duration) -> Timer {
  create_timer(
    Box::new(move || callback()),
    duration.as_millis() as u64, // TODO: use Duration
    false,
  )
}
