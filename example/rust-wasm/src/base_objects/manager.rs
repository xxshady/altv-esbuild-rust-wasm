use std::cell::RefCell;

use crate::{altv_events, logging::log_warn, BaseObject};

use super::{base_object_type::BaseObjectType, handle::BaseObjectHandle};

thread_local! {
  pub(crate) static MANAGER_INSTANCE: RefCell<Manager> = RefCell::new(Manager::new());
}

#[derive(Default)]
pub struct Manager {
  instances: Vec<BaseObjectHandle>,
}

impl Manager {
  pub fn new() -> Self {
    altv_events::add_handler(altv_events::Handler::baseObjectCreate(Box::new(|ctx| {
      if !matches!(ctx.base_object.btype, BaseObjectType::PLAYER) {
        return;
      }

      MANAGER_INSTANCE.with_borrow_mut(|manager| {
        manager.on_create(ctx.base_object.clone());
      });
    })));

    altv_events::add_handler(altv_events::Handler::baseObjectDestroy(Box::new(|ctx| {
      if !matches!(ctx.base_object.btype, BaseObjectType::PLAYER) {
        return;
      }

      MANAGER_INSTANCE.with_borrow_mut(|manager| {
        manager.on_destroy(ctx.base_object.clone());
      });
    })));

    Manager { instances: vec![] }
  }

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
