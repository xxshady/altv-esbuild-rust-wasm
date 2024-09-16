use serde::{Deserialize, Serialize};

use crate::wasm_imports;

use super::{
  as_base_object_type::AsBaseObjectType,
  base_object_type::BaseObjectType,
  handle::{BaseObjectGeneration, GenericBaseObjectHandle, BaseObjectId, BaseObjectHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
pub type VehicleHandle = BaseObjectHandle<VehicleType>;

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

  pub fn model(&self) -> u32 {
    wasm_imports::get_entity_model(&self.js_ref)
  }
}
