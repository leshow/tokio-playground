#![feature(never_type)]

mod interval;
mod intervalfuture;

use crate::{
    interval::Interval,
    intervalfuture::{IntervalFut, IntervalPrinter},
};
use futures::prelude::*;
use std::{thread, time::Duration};
use tokio::prelude::*;

fn main() {
    // main_poll()
    // main_sync()
    // main_tokio()
    main_tokio_ok()
}

fn main_sync() {
    let interval = Interval::from_millis(500);
    let duration = Duration::from_millis(100);

    for i in 1..=50 {
        println!("Iteration {}, counter {}", i, interval.get_counter());
        thread::sleep(duration);
    }
}

fn main_poll() {
    let mut fut = IntervalFut::new(Interval::from_millis(500));
    let duration = Duration::from_millis(100);

    for i in 1..=50 {
        match fut.poll() {
            Ok(Async::Ready(val)) => {
                println!("Iteration number {}, counter {}", i, val);
            }
            Ok(Async::NotReady) => (),
            Err(_) => unreachable!(),
        }
        thread::sleep(duration);
    }
}

fn main_tokio() {
    let fut = IntervalPrinter(IntervalFut::new(Interval::from_millis(500)));
    tokio::run(fut);
}

fn main_tokio_ok() {
    let fut = IntervalFut::new(Interval::from_millis(500));
    tokio::run(fut.map(|_| ()).map_err(|_| ()));
}
