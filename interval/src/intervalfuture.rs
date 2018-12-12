use crate::interval::Interval;
use futures::prelude::*;

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
            Ok(Async::NotReady)
        } else {
            self.last = cur;
            Ok(Async::Ready(cur))
        }
    }
}
