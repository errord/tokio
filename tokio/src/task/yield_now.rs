use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

doc_rt_core! {
    /// Yields execution back to the Tokio runtime.
    ///
    /// A task yields by awaiting on `yield_now()`, and may resume when that
    /// future completes (with no output.) The current task will be re-added as
    /// a pending task at the _back_ of the pending queue. Any other pending
    /// tasks will be scheduled. No other waking is required for the task to
    /// continue.
    ///
    /// See also the usage example in the [task module](index.html#yield_now).
    #[must_use = "yield_now does nothing unless polled/`await`-ed"]
    pub async fn yield_now() {
        /// Yield implementation
        struct YieldNow {
            yielded: bool,
        }

        impl Future for YieldNow {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
                debug!("yield_now entry");
                if self.yielded {
                    debug!("yield_now ready exit");
                    return Poll::Ready(());
                }

                self.yielded = true;
                // e: 重新执行，如不加则yield_now这个task不会被执行了
                // e: 1. context为scheduler(即main future)则立即执行,后面的task不会执行了
                // e: 2. context为task，重新放入task queue尾部，还能执行一轮就绪的task
                cx.waker().wake_by_ref();
                debug!("yield_now pending exit");
                Poll::Pending
            }
        }

        YieldNow { yielded: false }.await
    }
}
