use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
    let mut threads_vec = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));
    let barrier = Arc::new(Barrier::new(5));

    for i in 0..5 {
        let tasks = tasks.clone();
        let barrier = barrier.clone();
        let handle = thread::spawn(move || {
            // Tasks 1
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 1"));

            barrier.wait();
            // Task 2
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 2"));

            barrier.wait();
            // Task 3
            tasks
                .lock()
                .unwrap()
                .push(format!("Thread {i}, Completed its part on Task 3"));
        });
        threads_vec.push(handle);
    }

    for handle in threads_vec {
        handle.join().unwrap();
    }

    let task_lock = &*tasks.lock().unwrap();
    for contents in task_lock {
        println!("{contents}");
    }
}
