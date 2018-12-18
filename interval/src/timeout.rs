use futures::task::Task;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct Timeout {
    counter: Arc<AtomicUsize>,
    running: Arc<AtomicBool>,
    task: Arc<Mutex<Option<Task>>>, // i don't think Arc<Mutex<Task>> is necessary
}

impl Drop for Timeout {
    fn drop(&mut self) {
        println!("Timeout dropping...");
        self.running.store(false, Ordering::SeqCst);
    }
}

impl Timeout {
    pub fn from_millis(millis: u64) -> Self {
        let duration = Duration::from_millis(millis);
        let counter = Arc::new(AtomicUsize::new(0));
        let running = Arc::new(AtomicBool::new(true));

        let r_clone = running.clone();
        let c_clone = counter.clone();
        let task = Arc::new(Mutex::new(None));
        let t_clone: Arc<Mutex<Option<Task>>> = task.clone();

        thread::spawn(move || {
            while r_clone.load(Ordering::SeqCst) {
                thread::sleep(duration);
                let prev = c_clone.fetch_add(1, Ordering::SeqCst);
                println!("Timeout still alive, value was: {}", prev);

                if let Some(ref task) = *t_clone.lock().unwrap() {
                    task.notify();
                }
            }
        });

        Timeout {
            counter,
            running,
            task,
        }
    }

    pub fn set_task(&mut self, task: Task) {
        *self.task.lock().unwrap() = Some(task);
    }

    pub fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}
