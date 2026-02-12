use std::collections::VecDeque;
use std::task::{RawWaker, RawWakerVTable, Waker};
use std::{cell::RefCell, rc::Rc};

use super::task::Task;

pub struct WakerContext {
    task: Rc<Task>,
    queue_ptr: *const RefCell<VecDeque<Rc<Task>>>,
}

impl WakerContext {
    pub fn into_waker(self) -> Waker {
        let context = Box::new(self);
        let raw_walker = RawWaker::new(Box::into_raw(context) as *const (), &VTABLE);
        unsafe { Waker::from_raw(raw_walker) }
    }

    pub fn gen_waker(task: &Rc<Task>, queue: &RefCell<VecDeque<Rc<Task>>>) -> Waker {
        WakerContext {
            task: task.clone(),
            queue_ptr: queue as *const RefCell<VecDeque<Rc<Task>>>,
        }
        .into_waker()
    }
}

// --- RawWaker vtable functions ---

unsafe fn clone(data: *const ()) -> RawWaker {
    let context = unsafe { &*(data as *const WakerContext) };
    let new_context = Box::new(WakerContext {
        task: context.task.clone(),
        queue_ptr: context.queue_ptr,
    });
    RawWaker::new(Box::into_raw(new_context) as *const (), &VTABLE)
}

unsafe fn wake(data: *const ()) {
    let context = unsafe { Box::from_raw(data as *mut WakerContext) };
    let queue = unsafe { &*context.queue_ptr };
    queue.borrow_mut().push_back(context.task);
    // Box and context.task go out of scope here and are "freed"
}

unsafe fn wake_by_ref(data: *const ()) {
    let context = unsafe { &*(data as *const WakerContext) };
    let queue = unsafe { &*context.queue_ptr };
    queue.borrow_mut().push_back(context.task.clone());
    // Same as wake, but doesn't consume the waker
}

unsafe fn drop(data: *const ()) {
    unsafe { std::mem::drop(Box::from_raw(data as *mut WakerContext)) }
    // Free the memory if the waker is dropped without being called
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
