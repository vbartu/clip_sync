use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use super::task::Task;

pub struct RuntimeContext {
    pub queue: RefCell<VecDeque<Rc<Task>>>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        RuntimeContext {
            queue: RefCell::new(VecDeque::new()),
        }
    }
}
