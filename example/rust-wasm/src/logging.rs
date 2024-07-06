#[macro_export]
macro_rules! __log {
  ($($arg:tt)*) => {
    $crate::log_info(&format!($($arg)*))
  };
}
pub use __log as log;

#[macro_export]
macro_rules! __log_warn {
  ($($arg:tt)*) => {
    $crate::log_warn(&format!($($arg)*))
  };
}
pub use __log_warn as log_warn;
