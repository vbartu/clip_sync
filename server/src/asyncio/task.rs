use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Task {
    future: RefCell<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Task {
    pub fn new(future: Pin<Box<dyn Future<Output = ()>>>) -> Self {
        Task {
            future: RefCell::new(future),
        }
    }

    pub fn poll(&self, cx: &mut Context) -> Poll<()> {
        self.future.borrow_mut().as_mut().poll(cx)
    }
}
