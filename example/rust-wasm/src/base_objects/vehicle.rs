use super::{instance::BaseObject, scoped_instance::ScopedBaseObject};

#[derive(Clone)]
pub struct VehicleType;

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
