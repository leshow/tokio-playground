use futures::task::Task;
use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct Interval {
    counter: Arc<AtomicUsize>,
    running: Arc<AtomicBool>,
    task: Option<Task>, // i don't think Arc<Mutex<Task>> is necessary
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

        Interval {
            counter,
            running,
            task: None,
        }
    }

    pub fn set_task(&mut self, task: Task) {
        self.task = Some(task);
    }

    pub fn notify(&self) {
        if let Some(ref task) = self.task {
            // let task = task.lock().unwrap();
            task.notify();
        }
    }

    pub fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}
