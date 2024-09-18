use serde::{Deserialize, Serialize};

use crate::wasm_imports;

use super::{
  as_base_object_type::AsBaseObjectType,
  attached_to_scope::AttachedToScope,
  base_object_type::BaseObjectType,
  borrowed_instance::BorrowedBaseObject,
  class_traits::{
    self,
    entity::{Entity, SyncedEntity},
    world_object::WorldObject,
  },
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, GenericBaseObjectHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
pub type VehicleHandle = BaseObjectHandle<VehicleType>;

impl BorrowedBaseObject for VehicleHandle {}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct VehicleType;

impl AsBaseObjectType for VehicleType {
  fn as_base_object_type() -> BaseObjectType {
    BaseObjectType::Vehicle
  }
}

impl Vehicle {
  // TODO:
  // pub fn streamed_in<'scope>(scope: &'scope impl Scope) -> Vec<ScopedVehicle<'scope>> {
  //   let Vehicles = wasm_imports::get_streamed_in_vehicles();
  //   let vehicles: Vec<vehicleHandle> = from_value(vehicles).unwrap();

  //   vehicles
  //     .into_iter()
  //     .map(|handle| Self::get_by_handle(scope, handle).unwrap())
  //     .collect()
  // }
}

impl<'scope> class_traits::vehicle::Vehicle for ScopedVehicle<'scope> {}
impl<'scope> WorldObject for ScopedVehicle<'scope> {}
impl<'scope> Entity for ScopedVehicle<'scope> {}
impl<'scope> SyncedEntity<'scope> for ScopedVehicle<'scope> {}
