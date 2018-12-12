use crate::interval::Interval;
use futures::{prelude::*, try_ready};

pub struct IntervalFut {
    interval: Interval,
    last: usize,
}

impl IntervalFut {
    pub fn new(interval: Interval) -> Self {
        let last = interval.get_counter();
        IntervalFut { interval, last }
    }
}

impl Future for IntervalFut {
    type Error = !;
    type Item = usize;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let cur = self.interval.get_counter();
        if self.last == cur {
            self.interval.set_task(futures::task::current());
            Ok(Async::NotReady)
        } else {
            self.last = cur;
            self.interval.notify();
            Ok(Async::Ready(cur))
        }
    }
}

pub struct IntervalPrinter(pub IntervalFut);

impl Future for IntervalPrinter {
    type Error = ();
    type Item = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        // match self.0.poll() {
        //     Ok(Async::Ready(cur)) => {
        //         println!("Counter is: {}", cur);
        //         Ok(Async::Ready(()))
        //     }
        //     Ok(Async::NotReady) => Ok(Async::NotReady),
        //     Err(_) => Err(()),
        // }
        let cur = try_ready!(self.0.poll());
        println!("Counter is: {}", cur);
        Ok(Async::Ready(()))
    }
}
