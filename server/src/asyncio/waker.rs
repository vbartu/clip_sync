use std::rc::Rc;
use std::task::{RawWaker, RawWakerVTable, Waker};

use super::runtime_context::RuntimeContext;
use super::task::Task;

pub struct WakerContext {
    task: Rc<Task>,
    rctx: *const RuntimeContext,
}

impl WakerContext {
    pub fn gen_waker(task: &Rc<Task>, rctx: &RuntimeContext) -> Waker {
        let waker_context = Box::new(WakerContext {
            task: task.clone(),
            rctx: rctx as *const RuntimeContext,
        });
        let raw_walker =
            RawWaker::new(Box::into_raw(waker_context) as *const (), &VTABLE);
        unsafe { Waker::from_raw(raw_walker) }
    }

    pub fn extract_rctx(waker: &Waker) -> &RuntimeContext {
        unsafe {
            let data = *(waker as *const Waker as *const *const ());
            let context = &*(data as *const WakerContext);
            &*context.rctx
        }
    }
}

// --- RawWaker vtable functions ---

unsafe fn clone(data: *const ()) -> RawWaker {
    let context = unsafe { &*(data as *const WakerContext) };
    let new_context = Box::new(WakerContext {
        task: context.task.clone(),
        rctx: context.rctx,
    });
    RawWaker::new(Box::into_raw(new_context) as *const (), &VTABLE)
}

unsafe fn wake(data: *const ()) {
    let context = unsafe { Box::from_raw(data as *mut WakerContext) };
    let queue = unsafe { &(*context.rctx).queue };
    queue.borrow_mut().push_back(context.task);
    // Box and context.task go out of scope here and are "freed"
}

unsafe fn wake_by_ref(data: *const ()) {
    let context = unsafe { &*(data as *const WakerContext) };
    let queue = unsafe { &(*context.rctx).queue };
    queue.borrow_mut().push_back(context.task.clone());
    // Same as wake, but doesn't consume the waker
}

unsafe fn drop(data: *const ()) {
    unsafe { std::mem::drop(Box::from_raw(data as *mut WakerContext)) }
    // Free the memory if the waker is dropped without being called
}

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(clone, wake, wake_by_ref, drop);
