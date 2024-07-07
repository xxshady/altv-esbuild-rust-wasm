/// Scope where "borrowed" base objects (for example Player or Vehicle) are guaranteed to be valid
///
/// # Example
///
/// ```
/// // `context` implements Scope
/// altv::set_timeout(|context| {
///   // `streamed_in` needs scope to guarantee that
///   // returned players will not be used after destroy
///   let players = altv::Player::streamed_in(context);
///
///   altv::spawn_local(async {
///     dbg!(players); // error, `context` is dead at this moment
///   });
/// });
/// ```
pub trait Scope {}

impl Scope for crate::timers::TimerContext {}
impl Scope for ThisTickScope {}

struct ThisTickScope;

pub fn new_scope<R>(use_scope: impl for<'scope> FnOnce(&'scope ThisTickScope) -> R) -> R {
  use_scope(&ThisTickScope)
}
