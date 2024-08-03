use std::cell::RefCell;

use crate::{altv_events, log_info, logging::log_warn, BaseObject};

use super::{base_object_type::BaseObjectType, handle::BaseObjectHandle};

thread_local! {
  pub(crate) static MANAGER_INSTANCE: RefCell<Manager> = Default::default();
}

#[derive(Default)]
pub struct Manager {
  instances: Vec<BaseObjectHandle>,
}

impl Manager {
  pub fn on_create(&mut self, base_object: BaseObjectHandle) {
    self.instances.push(base_object);
  }

  pub fn on_destroy(&mut self, base_object: BaseObjectHandle) {
    let idx =
      self.instances.iter().enumerate().find_map(
        |(idx, el)| {
          if *el == base_object {
            Some(idx)
          } else {
            None
          }
        },
      );

    let Some(idx) = idx else {
      log_warn!("[on_destroy] failed to remove base object: {base_object:?}");
      return;
    };

    self.instances.swap_remove(idx);
  }

  pub fn is_handle_valid(&self, handle: &BaseObjectHandle) -> bool {
    self.instances.iter().any(|el| el == handle)
  }
}

pub fn init() {
  log_info("initializing base object manager");

  altv_events::add_handler(altv_events::Handler::baseObjectCreate(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      manager.on_create(ctx.base_object.clone());
    });
  })));

  altv_events::add_handler(altv_events::Handler::baseObjectRemove(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      manager.on_destroy(ctx.base_object.clone());
    });
  })));
}
