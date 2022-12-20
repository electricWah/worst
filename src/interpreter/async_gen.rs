
//! Generator/coroutine thing using async

use std::rc::Rc;
use std::cell::Cell;
use std::pin::Pin;
use std::future::Future;
use std::rc::Weak;
use std::task::Context;
use std::task::Poll;
use std::task::Waker;
use std::task::Wake;
use std::sync::Arc;

/// Outside view of a generator:
/// to get the next value, poll the task, take the value out of chan
pub struct Generator<T> {
    chan: Rc<Cell<Option<T>>>,
    task: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

/// Inside view of a generator: put a value in chan and then wait
pub struct Ctx<T> {
    chan: Weak<Cell<Option<T>>>,
}
/// Pollable value for Ctx to await when yielding
struct CtxYield {
    ready: bool,
}

/// A Waker that does nothing.
struct NoWaker;

impl<T> Generator<T> {
    /// Create a new Generator from the given function.
    pub fn new<O: 'static + Future<Output=()>,
               F: 'static + FnOnce(Ctx<T>) -> O>(f: F) -> Self {
        let chan = Rc::new(Cell::new(None));
        let task = Box::pin(f(Ctx { chan: Rc::downgrade(&chan) }));
        let waker = Arc::new(NoWaker).into();
        Generator { chan, task, waker }
    }
}

impl<T> Iterator for Generator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let mut cx = Context::from_waker(&self.waker);
        let polled = self.task.as_mut().poll(&mut cx);
        let v = self.chan.take();
        match polled {
            Poll::Pending => {
                Some(v.expect("Generator awaited without yielding value"))
            },
            Poll::Ready(()) => {
                if v.is_some() {
                    panic!("Generator finished early");
                }
                None
            },
        }
    }
}

impl<T> Ctx<T> {
    /// Yield a value to the outer Generator.
    pub async fn yield_(&self, v: T) {
        let rc = self.chan.upgrade().expect("Ctx outlived Generator");
        if rc.replace(Some(v)).is_some() {
            panic!("Generator resumed without taking value");
        }
        (CtxYield { ready: false }).await;
    }
}

impl Wake for NoWaker {
    fn wake(self: Arc<Self>) {
        panic!(".await used inside generator, but not with Ctx::yield_?");
    }
}

impl Future for CtxYield {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.ready {
            Poll::Ready(())
        } else {
            self.get_mut().ready = true;
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        for x in Generator::new(|ctx: Ctx<usize>| async move { }) {
            panic!("not empty");
        }
    }

    #[test]
    fn counter() {
        let mut i = 0;
        for x in Generator::new(|ctx| async move {
            for i in 0..10 {
                ctx.yield_(i).await;
            }
        }) {
            assert_eq!(x, i);
            i = i + 1;
        }
        assert_eq!(i, 10);
    }

}

