#![feature(generators, generator_trait, conservative_impl_trait, i128_type)]

use std::ops::{Generator, GeneratorState};

fn main() {
    let mut fn1 = 0;
    let mut fn2 = 1;
    let mut fibber = || loop {
        let t = fn1 + fn2;
        fn1 = fn2;
        fn2 = t;
        println!("{:?}", t);
        yield t;
    };

    fibber.resume();
    fibber.resume();
    fibber.resume();
    fibber.resume();
    fibber.resume();

    let mut gfib = Fib::new();

    for _ in 0..10 {
        if let GeneratorState::Yielded(val) = gfib.resume() {
            println!("gfib -- {:?}", val);
        }
    }
}

struct Fib {
    f: u128,
    s: u128,
    done: bool,
}

impl Fib {
    pub fn new() -> Self {
        Self {
            f: 0,
            s: 1,
            done: false,
        }
    }
}

impl Generator for Fib {
    type Yield = u128;
    type Return = u128;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        let a = self.f;
        let b = self.s;

        let (res, overflow) = a.overflowing_add(b);
        self.done = overflow;
        if overflow {
            GeneratorState::Complete(b)
        } else {
            self.f = b;
            self.s = res;
            GeneratorState::Yielded(res)
        }
    }
}

fn fibbers() -> impl Generator {
    let mut fn1 = 0;
    let mut fn2 = 1;

    let fibs = move || loop {
        fn1 = fn2;
        fn2 = fn1 + fn2;
        println!("{:?}", fn2);
        yield fn1;
    };
    return fibs;
}

// match generator.resume() {
//     GeneratorState::Yielded(1) => {
//         println!("1");
//     }
//     _ => panic!("unexpected value from resume"),
// }
// match generator.resume() {
//     GeneratorState::Complete("foo") => {
//         println!("foo");
//     }
//     _ => panic!("unexpected value from resume"),
// }
// the following is broken, rustc cannot compile .resume()
// fn test_fibbers() {
//     let mut fibbers2 = fibbers();
//     fibbers.resume();
//     fibbers.resume();
//     fibbers.resume();
// }
// match generator.resume() {
//     GeneratorState::Yielded(1) => {
//         println!("1");
//     }
//     _ => panic!("unexpected value from resume"),
// }
// match generator.resume() {
//     GeneratorState::Complete("foo") => {
//         println!("foo");
//     }
//     _ => panic!("unexpected value from resume"),
// }
