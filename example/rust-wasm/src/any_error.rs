use std::fmt::{Debug, Display, Formatter};

use crate::{wasm_imports, logging::log_info};

/// Any error, right now it's only needed to provide backtraces (if "error_backtrace" feature is enabled) when returning any error from callbacks (for example from timers).
/// Works similarly to [`anyhow::Error`](https://docs.rs/anyhow/latest/anyhow/struct.Error.html).
/// Use `Display` trait (for example `.to_string()`) to read error message and `Debug` trait to read backtrace (if "error_backtrace" feature is enabled).
///
/// # Examples
/// ```
/// fn error_with_backtrace() -> Result<(), AnyError> {
///   // ParseIntError will be automatically converted to `AnyError`
///   // with backtrace (if "error_backtrace" feature is enabled)
///   "".parse::<i32>()?;
///   Ok(())
/// }
/// ```
pub struct AnyError {
  message: String,

  // TODO: add CI test & error_backtrace feature
  // #[cfg(feature = "error_backtrace")]
  backtrace: String,
}

impl AnyError {
  pub fn from_anything<T: Display>(value: T) -> Self {
    Self {
      message: value.to_string(),
      // #[cfg(feature = "error_backtrace")]
      backtrace: capture_backtrace(),
    }
  }
}

impl Debug for AnyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)?;

    // #[cfg(feature = "error_backtrace")]
    write!(f, "\nbacktrace: {}", self.backtrace)
  }
}

impl Display for AnyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<E> From<E> for AnyError
where
  E: std::error::Error + Send + Sync + 'static,
{
  #[cold]
  fn from(error: E) -> Self {
    Self {
      message: error.to_string(),

      // #[cfg(feature = "error_backtrace")]
      backtrace: capture_backtrace(),
    }
  }
}

// #[cfg(feature = "error_backtrace")]
fn capture_backtrace() -> String {
  let js_stack = wasm_imports::Error::new().stack();

  // removing "Error" line
  // TODO: remove unneeded frames (.js & Error::new)
  let idx = js_stack
    .find('\n')
    .expect("js Error stack must include \\n after 'Error'");
  // TODO: avoid .to_string(), tho does it really matter here?
  let backtrace = js_stack[idx..].to_string();
  // TEST
  log_info!("captured backtrace: {backtrace}");
  backtrace
}
