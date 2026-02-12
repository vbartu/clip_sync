use std::collections::VecDeque;
use std::future::Future;
use std::task::{Context, Poll};
use std::{cell::RefCell, rc::Rc};

use super::task::Task;
use super::waker::WakerContext;

pub struct Executor {
    queue: RefCell<VecDeque<Rc<Task>>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn create_task<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Task::new(Box::pin(future));
        self.queue.borrow_mut().push_front(Rc::new(task));
    }

    pub fn run(&mut self) {
        loop {
            match self.queue.borrow_mut().pop_back() {
                None => return, // No more tasks
                Some(task) => {
                    let waker = WakerContext::gen_waker(&task, &self.queue);
                    let mut context = Context::from_waker(&waker);
                    match task.poll(&mut context) {
                        Poll::Pending => {
                            self.queue.borrow_mut().push_front(task)
                        }
                        Poll::Ready(_) => println!("Task completed"),
                    };
                }
            }
        }
    }
}
