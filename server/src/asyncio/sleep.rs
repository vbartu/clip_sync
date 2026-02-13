use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use super::waker::WakerContext;

pub struct Sleep {
    instant: Instant,
}

impl Sleep {
    fn delay(delay: Duration) -> Self {
        Sleep {
            instant: Instant::now() + delay,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.instant <= Instant::now() {
            Poll::Ready(())
        } else {
            let rctx = WakerContext::extract_rctx(cx.waker());
            rctx.timer
                .borrow_mut()
                .insert(self.instant, cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn sleep(delay: Duration) -> Sleep {
    Sleep::delay(delay)
}
