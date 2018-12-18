use crate::timeout::Timeout;
use futures::prelude::*;
use tokio::prelude::*;

pub struct TimeoutStream {
    timeout: Timeout,
    last: usize,
}

impl TimeoutStream {
    pub fn new(timeout: Timeout) -> Self {
        let last = timeout.get_counter();
        TimeoutStream { timeout, last }
    }
}

impl Stream for TimeoutStream {
    type Error = ();
    type Item = usize;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let cur = self.timeout.get_counter();
        if cur == self.last {
            self.timeout.set_task(futures::task::current());
            Ok(Async::NotReady)
        } else {
            self.last = cur;
            Ok(Async::Ready(Some(cur)))
        }
    }
}

pub fn run_timeout_stream() {
    let fut = TimeoutStream::new(Timeout::from_millis(500))
        .map(|f| f * 2)
        .take(10)
        .for_each(|cur| {
            println!("Stream counter: {}", cur);
            futures::future::ok(())
        });
    tokio::run(fut)
}
