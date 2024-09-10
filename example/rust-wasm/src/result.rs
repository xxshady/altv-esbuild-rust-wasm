use std::fmt::{Debug, Display};

use crate::any_error::AnyError;

pub type AnyVoidResult = Result<(), AnyError>;

pub trait IntoVoidResult {
  fn into_void_result(self) -> AnyVoidResult;
}

impl IntoVoidResult for () {
  fn into_void_result(self) -> AnyVoidResult {
    Ok(())
  }
}

impl<E: Display + Debug> IntoVoidResult for Result<(), E> {
  fn into_void_result(self) -> AnyVoidResult {
    match self {
      Ok(()) => Ok(()),
      Err(err) => Err(AnyError::from_anything(err)),
    }
  }
}
