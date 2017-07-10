extern crate futures;
extern crate tokio_core;

use futures::{Future, Poll, Async};
use tokio_core::reactor::Core;

struct Poller;

impl Future for Poller {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(()))
    }
}

#[test]
pub fn poller() {
    let mut core = Core::new().unwrap();
    let poller = Poller {};
    assert_eq!(Ok(()), core.run(poller));
}

struct PollMeNTimes<T> {
    n: u64,
    answer: T,
}

impl<T: Copy> Future for PollMeNTimes<T> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("poll: {}", self.n);
        if self.n != 0 {
            self.n -= 1;
            futures::task::current().notify();
            Ok(Async::NotReady)
        } else {
            Ok(Async::Ready(self.answer))
        }
    }
}

#[test]
pub fn poll_me_0_times() {
    let mut core = Core::new().unwrap();

    let pm0t = PollMeNTimes { n: 0, answer: 50 };
    assert_eq!(Ok(50), core.run(pm0t));
}

#[test]
pub fn poll_me_3_times() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let pm3t = PollMeNTimes { n: 3, answer: 42 };

    handle.spawn(Poller);
    handle.spawn(Poller);
    handle.spawn(Poller);

    assert_eq!(Ok(42), core.run(pm3t));
}

fn main() {
    println!("Hello, world!");
}
