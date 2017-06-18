extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn sync_prime() {
    if is_prime(BIG_PRIME) {
        println!("Prime");
    } else {
        println!("Not a prime");
    }
}

fn main() {
    let pool = CpuPool::new_num_cpus();

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Created the future.");
    if prime_future.wait().unwrap() {
        // wait for the future to return a value
        println!("Future prime.");
    } else {
        println!("Future not prime");
    }

    let timer = Timer::default();
    let timeout = timer.sleep(Duration::from_millis(1000)).then(|_| Err(()));

    let prime = pool.spawn_fn(|| Ok(is_prime(BIG_PRIME)));

    let winner = timeout.select(prime).map(|(win, _)| win);

    match winner.wait() {
        Ok(true) => println!("prime"),
        Ok(false) => println!("Not prime"),
        Err(_) => println!("Timed out"),
    }
}

