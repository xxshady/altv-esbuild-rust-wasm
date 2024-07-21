use super::{player::Player, vehicle::Vehicle};

pub enum AnyBaseObject {
  Player(Player),
  Vehicle(Vehicle),
}
