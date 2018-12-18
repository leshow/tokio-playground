use futures::prelude::*;
use tokio::prelude::*;

pub struct MyOk<T>(Option<T>);

impl<T> MyOk<T> {
    pub fn new(t: T) -> Self {
        MyOk(Some(t))
    }
}

impl<T> Future for MyOk<T> {
    type Error = ();
    type Item = T;

    fn poll(&mut self) -> Poll<T, ()> {
        if let Some(t) = self.0.take() {
            Ok(Async::Ready(t))
        } else {
            Ok(Async::NotReady)
        }
    }
}

pub fn main() {
    let name = String::from("Alice");
    let future = MyOk::new(name).and_then(|name| {
        println!("Name: {}", name);
        MyOk::new(())
    });

    tokio::run(future)
}

pub fn range() {
    tokio::run(stream::iter_ok((1..=10)).collect().and_then(|val| {
        println!("{:?}", val);
        future::ok(())
    }))
}