use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
fn try_max(stack: Option<usize>, label: &str) {
    let stop = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();
    loop {
        let s = stop.clone();
        let mut b = thread::Builder::new();
        if let Some(sz) = stack {
            b = b.stack_size(sz);
        }
        match b.spawn(move || {
            while !s.load(Ordering::Relaxed) {
                thread::yield_now();
            }
        }) {
            Ok(h) => handles.push(h),
            Err(e) => {
                println!("{label}: created {} threads, then: {}", handles.len(), e);
                break;
            }
        }
    }
    stop.store(true, Ordering::Relaxed);
    for h in handles {
        h.join().unwrap();
    }
}
fn main() {
    try_max(None, "2 MiB stack (default)");
    try_max(Some(64 * 1024), "64 KiB stack");
}
