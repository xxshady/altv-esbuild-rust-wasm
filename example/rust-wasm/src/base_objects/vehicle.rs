use super::{
  base_object_type::BaseObjectType, handle::BaseObjectHandle, instance::BaseObject,
  manager::MANAGER_INSTANCE, scope::Scope, scoped_instance::ScopedBaseObject,
  unscoped_instance::UnscopedBaseObject,
};

#[derive(Clone)]
pub struct VehicleType;

pub type Vehicle = BaseObject<VehicleType>;
pub type ScopedVehicle<'scope> = ScopedBaseObject<'scope, VehicleType>;
pub type UnscopedVehicle<'scope> = UnscopedBaseObject<VehicleType>;

impl Vehicle {
  pub fn get_by_id<'scope>(scope: &'scope impl Scope, id: u32) -> Option<ScopedVehicle<'scope>> {
    MANAGER_INSTANCE.with_borrow(|manager| {
      let vehicle = BaseObject::new_by_handle(
        manager,
        BaseObjectHandle {
          id,
          btype: BaseObjectType::VEHICLE,
        },
      )?;
      Some(scope.attach_base_object(vehicle))
    })
  }
}
