use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

struct Interval {
    counter: Arc<AtomicUsize>,
    running: Arc<AtomicBool>,
}

impl Drop for Interval {
    fn drop(&mut self) {
        println!("Interval dropping...");
        self.running.store(false, Ordering::SeqCst);
    }
}

impl Interval {
    pub fn from_millis(millis: u64) -> Self {
        let duration = Duration::from_millis(millis);
        let counter = Arc::new(AtomicUsize::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let r_clone = running.clone();
        let c_clone = counter.clone();

        thread::spawn(move || {
            while r_clone.load(Ordering::SeqCst) {
                thread::sleep(duration);
                let prev = c_clone.fetch_add(1, Ordering::SeqCst);
                println!("Interval still alive, value was: {}", prev);
            }
        });

        Interval { counter, running }
    }

    pub fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

fn main() {
    let interval = Interval::from_millis(500);
    let duration = Duration::from_millis(100);

    for i in 1..=50 {
        println!("Iteration {}, counter {}", i, interval.get_counter());
        thread::sleep(duration);
    }
}
