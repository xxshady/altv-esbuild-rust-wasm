use serde::{Deserialize, Serialize};

use crate::wasm_imports;

use super::{
  base_object_type::BaseObjectType,
  handle::{BaseObjectGeneration, BaseObjectHandle, BaseObjectId, BaseObjectSpecificHandle},
  instance::BaseObject,
  manager::MANAGER_INSTANCE,
  scope::Scope,
  scoped_instance::ScopedBaseObject,
};

pub type Vehicle = BaseObject<VehicleHandle>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleHandle>;

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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct VehicleHandle {
  id: BaseObjectId,
  generation: BaseObjectGeneration,
}

impl BaseObjectSpecificHandle for VehicleHandle {
  fn to_base(self) -> BaseObjectHandle {
    BaseObjectHandle {
      btype: BaseObjectType::Vehicle,
      id: self.id,
      generation: self.generation,
    }
  }

  fn attach_to<'scope>(self, scope: &'scope impl Scope) -> Option<ScopedVehicle<'scope>> {
    MANAGER_INSTANCE.with_borrow(|manager| {
      let vehicle = BaseObject::new_by_handle(manager, self)?;
      Some(scope.attach_base_object(vehicle))
    })
  }
}
