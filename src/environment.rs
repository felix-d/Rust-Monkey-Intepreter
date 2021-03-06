use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::object::Object;

#[derive(Debug, Clone)]
pub(crate) struct Environment(Rc<RefCell<InnerEnvironment>>);

#[derive(Debug)]
struct InnerEnvironment {
    store: HashMap<String, Object>,
    outer: Option<Environment>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Environment(Rc::new(RefCell::new(InnerEnvironment {
            store: HashMap::new(),
            outer: None,
        })))
    }

    pub(crate) fn new_enclosed_environment(outer: Environment) -> Self {
        Environment(Rc::new(RefCell::new(InnerEnvironment {
            store: HashMap::new(),
            outer: Some(outer),
        })))
    }

    pub(crate) fn get(&self, key: &str) -> Option<Object> {
        let env = self.0.borrow();

        env.store.get(key).map(Clone::clone)
            .or_else(|| env.outer.as_ref().and_then(|outer| outer.get(key)))
    }

    pub(crate) fn set(&mut self, key: String, val: Object) {
        self.0.borrow_mut().store.insert(key, val);
    }
}
