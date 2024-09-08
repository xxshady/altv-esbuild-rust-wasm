use std::fmt::{Debug, Display, Formatter};

use crate::wasm_imports;

/// Custom Error, right now it's only needed to provide backtraces (if "error_backtrace" feature is enabled) when returning errors from event or timer callbacks.
/// Works similarly to [`anyhow::Error`](https://docs.rs/anyhow/latest/anyhow/struct.Error.html).
/// # Examples
/// ```
/// fn test_error_with_backtrace() -> Result<(), Error> {
///   // ParseIntError will be automatically converted to Error
///   // with backtrace (if "error_backtrace" feature is enabled)
///   "".parse::<i32>()?;
///   Ok(())
/// }
/// ```
pub struct Error {
  message: String,

  // TODO: add CI test & error_backtrace feature
  // #[cfg(feature = "error_backtrace")]
  backtrace: String,
}

impl Debug for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)?;

    // #[cfg(feature = "error_backtrace")]
    write!(f, "\nbacktrace: {}", self.backtrace)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<E> From<E> for Error
where
  E: std::error::Error + Send + Sync + 'static,
{
  #[cold]
  fn from(error: E) -> Self {
    Self {
      message: error.to_string(),

      // #[cfg(feature = "error_backtrace")]
      backtrace: {
        let js_stack = wasm_imports::Error::new().stack();

        // removing "Error" line
        // TODO: remove unneeded frames (.js & Error::new)
        let idx = js_stack
          .find('\n')
          .expect("js Error stack must include \\n after 'Error'");
        // TODO: avoid .to_string(), tho does it really matter here?
        js_stack[idx..].to_string()
      },
    }
  }
}
