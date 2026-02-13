use std::collections::{BTreeMap, VecDeque};
use std::task::Waker;
use std::time::Instant;
use std::{cell::RefCell, rc::Rc};

use super::task::Task;

pub struct RuntimeContext {
    pub queue: RefCell<VecDeque<Rc<Task>>>,
    pub timer: RefCell<BTreeMap<Instant, Waker>>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        RuntimeContext {
            queue: RefCell::new(VecDeque::new()),
            timer: RefCell::new(BTreeMap::new()),
        }
    }
}
