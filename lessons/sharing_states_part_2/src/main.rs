use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

struct File {
    text: Vec<String>,
}
fn main() {
    let file = Arc::new(Mutex::new(File { text: vec![] }));
    let mut thread_vec = vec![];

    for i in 0..10 {
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hello from Thread {i}"));
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }
}
