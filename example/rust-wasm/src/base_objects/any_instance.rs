use super::{base_object_type::BaseObjectType, instance::BaseObject, player::Player};

pub enum AnyBaseObject {
  Player(Player),
}

impl From<&AnyBaseObject> for &Player {
  fn from<'a>(value: &'a AnyBaseObject) -> &'a Player {
    match value {
      AnyBaseObject::Player(player) => player,
    }
  }
}
