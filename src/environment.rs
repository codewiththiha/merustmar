use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn reassign(&mut self, name: String, val: Object) -> Result<(), String> {
        use std::collections::hash_map::Entry;
        match self.store.entry(name.clone()) {
            Entry::Occupied(mut e) => {
                e.insert(val);
                Ok(())
            }
            Entry::Vacant(_) => {
                if let Some(ref outer) = self.outer {
                    outer.borrow_mut().reassign(name, val)
                } else {
                    Err(format!("Cannot reassign undefined identifier: {}", name))
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store
            .get(name)
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|o| o.borrow().get(name)))
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
