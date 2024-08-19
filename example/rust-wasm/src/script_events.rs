use std::{borrow::Cow, cell::RefCell, collections::HashMap};

use crate::{base_objects::scope::Scope, logging::log_error};

use js_sys::{ArrayBuffer, Uint8Array};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use crate::{
  id_provider::{Id, IdProvider},
  logging::log_info,
  wasm_imports,
};

pub struct RawScriptEventContext {
  args: RawScriptEventArgs,
}

#[derive(Debug)]
pub struct RawScriptEventArgs {
  raw: JsValue,
}

impl RawScriptEventArgs {
  /// For Rust-to-Rust deserialization
  ///
  /// See also: [`Self::deserialize_js`]
  pub fn deserialize<T>(&self) -> Result<T, DeserializationError>
  where
    T: DeserializeOwned,
  {
    let raw = self.raw.clone();

    let (bytes,): (ByteBuf,) = from_value(raw).map_err(DeserializationError::Serde)?;

    // TODO: let user decide what crate to use for bytes de(serialization)
    bincode::deserialize(&bytes).map_err(|err| DeserializationError::Bincode(*err))
  }

  /// For JS-to-Rust deserialization
  ///
  /// See also: [`Self::deserialize`]
  pub fn deserialize_js<T>(&self) -> Result<T, serde_wasm_bindgen::Error>
  where
    T: DeserializeOwned,
  {
    from_value(self.raw.clone())
  }
}

impl Scope for RawScriptEventContext {}

#[derive(Debug)]
pub enum DeserializationError {
  Serde(serde_wasm_bindgen::Error),
  Bincode(bincode::ErrorKind),
}

pub struct ScriptEventContext<'a, T: DeserializeOwned> {
  pub data: &'a T,

  /// Disallow creation of this struct from outside
  _private: (),
}

impl<'a, T: DeserializeOwned> Scope for ScriptEventContext<'a, T> {}

thread_local! {
  static MANAGER_INSTANCE: RefCell<Manager<'static>> = Default::default();
}

type Handler = Box<dyn FnMut(&RawScriptEventContext)>;
type HandlerId = Id;

#[derive(Default)]
struct Manager<'a> {
  local_handlers: HashMap<Cow<'a, str>, HashMap<HandlerId, Handler>>,
  remote_handlers: HashMap<Cow<'a, str>, HashMap<HandlerId, Handler>>,
  handler_id_provider: IdProvider,
}

#[derive(Deserialize, Debug)]
struct Event {
  local: bool,
  name: String,
  #[serde(with = "serde_wasm_bindgen::preserve")]
  args: JsValue,
}

#[wasm_bindgen]
pub fn on_script_local_event(event: JsValue) {
  let event: Event = from_value(event).unwrap();
  // log_info!("on_script_local_event {event:?}");

  MANAGER_INSTANCE.with_borrow_mut(|instance| {
    let event_name: Cow<'_, str> = event.name.into();
    let Some(handlers) = instance.local_handlers.get_mut(&event_name) else {
      return;
    };

    let context = RawScriptEventContext {
      args: RawScriptEventArgs { raw: event.args },
    };
    for handler in handlers.values_mut() {
      handler(&context);
    }
  });
}

pub fn add_local_handler_raw(
  event_name: impl Into<Cow<'static, str>>,
  handler: impl FnMut(&RawScriptEventContext) + 'static,
) -> HandlerId {
  let event_name = event_name.into();

  MANAGER_INSTANCE.with_borrow_mut(move |instance| {
    let handlers = instance.local_handlers.entry(event_name).or_default();
    let id = instance.handler_id_provider.next();
    handlers.insert(id, Box::new(handler));
    id
  })
}

pub fn add_local_handler<T: DeserializeOwned>(
  event_name: impl Into<Cow<'static, str>>,
  mut handler: impl FnMut(&ScriptEventContext<'_, T>) + 'static,
) -> HandlerId {
  let event_name = event_name.into();

  MANAGER_INSTANCE.with_borrow_mut(move |instance| {
    let handlers = instance
      .local_handlers
      .entry(event_name.clone())
      .or_default();
    let id = instance.handler_id_provider.next();
    handlers.insert(
      id,
      Box::new(move |context| {
        // TODO: deserialize once for all rust handlers
        match context.args.deserialize::<T>() {
          Ok(data) => {
            let context = ScriptEventContext {
              data: &data,
              _private: (),
            };
            handler(&context)
          }
          Err(err) => {
            log_error!("Failed to deserialize data for event: {event_name}, error: {err:?}");
          }
        }
      }),
    );
    id
  })
}

pub fn remove_local_handler(handler_id: HandlerId) {
  todo!()
}

/// For Rust-to-Rust serialization
///
/// See also: [`emit_js`]
pub fn emit(
  event_name: &str,
  data: &dyn erased_serde::Serialize,
) -> Result<(), bincode::ErrorKind> {
  let bytes = bincode::serialize(data).map_err(|err| *err)?;
  let array = Uint8Array::from(bytes.as_slice());
  wasm_imports::emit_local_event_rust(event_name, array.buffer());
  Ok(())
}

/// For JS-to-Rust serialization
///
/// See also: [`emit`]
pub fn emit_js(
  event_name: &str,
  args: &[&dyn erased_serde::Serialize],
) -> Result<(), serde_wasm_bindgen::Error> {
  let js_value = to_value(args)?;

  wasm_imports::emit_local_event_js(event_name, js_value);

  Ok(())
}

#[wasm_bindgen]
pub fn test_script_events() {
  // let event_name = String::from("test");

  // type TestData = (i32, bool, Vec<i32>);

  // add_local_handler::<TestData>(event_name.clone(), |context| {
  //   let data = context.data;
  //   log_info!("data: {data:?}");
  // });

  // let data: TestData = (i32::MAX, true, vec![1, 2, 3]);
  // emit(&event_name, &data).unwrap();

  // let js_event_name = event_name + "_js";

  // add_local_handler_raw(js_event_name.clone(), |context| {
  //   let (args,): (TestData,) = context.args.deserialize_js().unwrap();
  //   log_info!("args: {args:?}");
  // });

  // // TODO: FIX THIS
  // emit_js(&js_event_name, &[&data]).unwrap();
}
