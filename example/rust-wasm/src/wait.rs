use crate::{base_objects::scope::Scope, helpers::net_time, timers::set_timeout};
use std::{
  cell::Cell,
  future::{poll_fn, Future},
  ops::Deref,
  rc::Rc,
  task::Poll,
  time::Duration,
};

pub fn wait(duration: Duration) -> impl Future {
  let dest = net_time() + duration;
  let mut timer_was_set = false;

  poll_fn(move |cx| {
    if net_time() >= dest {
      return Poll::Ready(());
    }
    if timer_was_set {
      return Poll::Pending;
    }
    timer_was_set = true;

    let waker = cx.waker().clone();
    set_timeout(
      |_| {
        waker.wake();
      },
      duration,
    );

    Poll::Pending
  })
}

pub struct WaitForContext(());
impl Scope for WaitForContext {}

#[must_use]
pub struct Success(bool);

impl Deref for Success {
  type Target = bool;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub fn wait_for(
  mut callback: impl FnMut(&WaitForContext) -> bool,
  timeout: Duration,
) -> impl Future<Output = Success> {
  let timeout_dest = net_time() + timeout;
  let expecting_poll = Rc::new(Cell::new(true));

  poll_fn(move |cx| {
    if !expecting_poll.get() {
      return Poll::Pending;
    }
    expecting_poll.set(false);

    if callback(&WaitForContext(())) {
      return Poll::Ready(Success(true));
    }

    if net_time() >= timeout_dest {
      return Poll::Ready(Success(false));
    }

    let waker = cx.waker().clone();
    let expecting_poll = expecting_poll.clone();
    set_timeout(
      move |_| {
        expecting_poll.set(true);
        waker.wake();
      },
      Duration::ZERO,
    );
    return Poll::Pending;
  })
}
