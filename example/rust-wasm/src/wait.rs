use crate::timers::set_timeout;
use std::{
  future::{poll_fn, Future},
  task::Poll,
};
use web_time::{Duration, SystemTime};

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
    set_timeout(
      Box::new(|_| {
        waker.wake();
      }),
      duration,
    );

    Poll::Pending
  })
}
