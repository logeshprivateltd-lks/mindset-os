//! Reactive local state container with dirty notification.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::core::ElementId;
use std::cell::RefCell;
use std::rc::Rc;

/// A reactive state container bound to a specific UI node or subtree.
#[derive(Debug, Clone)]
pub struct StateHandle<T> {
    id: ElementId,
    value: Rc<RefCell<T>>,
    dirty: Rc<RefCell<bool>>,
}

impl<T: Clone> StateHandle<T> {
    pub fn new(id: ElementId, initial: T) -> Self {
        Self {
            id,
            value: Rc::new(RefCell::new(initial)),
            dirty: Rc::new(RefCell::new(false)),
        }
    }

    pub fn id(&self) -> ElementId {
        self.id
    }

    pub fn get(&self) -> T {
        self.value.borrow().clone()
    }

    pub fn set(&self, new_val: T) {
        *self.value.borrow_mut() = new_val;
        *self.dirty.borrow_mut() = true;
    }

    pub fn is_dirty(&self) -> bool {
        *self.dirty.borrow()
    }

    pub fn clear_dirty(&self) {
        *self.dirty.borrow_mut() = false;
    }
}
