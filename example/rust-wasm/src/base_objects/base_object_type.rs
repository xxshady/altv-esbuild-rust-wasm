use serde_repr::{Deserialize_repr, Serialize_repr};

// TODO: generate it from sdk or TS typings
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum BaseObjectType {
  PLAYER,
  VEHICLE,
  PED,
  OBJECT,
  BLIP,
  WEBVIEW,
  VOICE_CHANNEL,
  COLSHAPE,
  CHECKPOINT,
  WEBSOCKET_CLIENT,
  HTTP_CLIENT,
  AUDIO,
  AUDIO_OUTPUT,
  AUDIO_OUTPUT_WORLD,
  AUDIO_OUTPUT_ATTACHED,
  AUDIO_OUTPUT_FRONTEND,
  RML_ELEMENT,
  RML_DOCUMENT,
  LOCAL_PLAYER,
  LOCAL_OBJECT,
  VIRTUAL_ENTITY,
  VIRTUAL_ENTITY_GROUP,
  MARKER,
  TEXT_LABEL,
  LOCAL_PED,
  LOCAL_VEHICLE,
  AUDIO_FILTER,
  CONNECTION_INFO,
  CUSTOM_TEXTURE,
  FONT,
  SIZE,
}
