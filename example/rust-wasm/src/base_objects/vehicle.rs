use super::{
  any_instance::{AnyBaseObject},
  instance::BaseObject,
  scoped_instance::ScopedBaseObject,
};

pub struct VehicleType;

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
