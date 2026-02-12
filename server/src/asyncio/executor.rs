use std::future::Future;
use std::rc::Rc;
use std::task::{Context, Poll};

use super::runtime_context::RuntimeContext;
use super::task::Task;
use super::waker::WakerContext;

pub struct Executor {
    rctx: RuntimeContext,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            rctx: RuntimeContext::new(),
        }
    }

    pub fn create_task<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Task::new(Box::pin(future));
        self.rctx.queue.borrow_mut().push_front(Rc::new(task));
    }

    fn execute_pending_tasks(&mut self) {
        loop {
            let task_opt = self.rctx.queue.borrow_mut().pop_back();
            if let Some(task) = task_opt {
                let waker = WakerContext::gen_waker(&task, &self.rctx);
                let mut cx = Context::from_waker(&waker);
                match task.poll(&mut cx) {
                    Poll::Pending => {}
                    Poll::Ready(_) => {}
                }
            } else {
                break;
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.execute_pending_tasks();
            if self.rctx.queue.borrow().is_empty() {
                break;
            }
        }
    }
}
