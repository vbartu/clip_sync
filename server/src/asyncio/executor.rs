use std::future::Future;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Instant;

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

    fn check_timer_tasks(&mut self) {
        loop {
            let next_time = {
                match self.rctx.timer.borrow().first_key_value() {
                    None => return, // No entries left
                    Some((next_time, _)) => *next_time,
                }
            };
            if next_time > Instant::now() {
                break; // to soon for next task
            }
            let (_, waker) =
                { self.rctx.timer.borrow_mut().pop_first().unwrap() };
            waker.wake();
        }
    }

    fn execute_pending_tasks(&mut self) {
        loop {
            let task_opt = {
                let mut queue = self.rctx.queue.borrow_mut();
                queue.pop_back()
            };
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
            self.check_timer_tasks();
            self.execute_pending_tasks();
            if let Some((next_time, _)) =
                self.rctx.timer.borrow().first_key_value()
            {
                std::thread::sleep(*next_time - Instant::now());
            } else {
                // No scheduled timer tasks, exit
                break;
            }
        }
    }
}
