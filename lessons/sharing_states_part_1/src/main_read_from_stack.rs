use std::thread;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    let t = thread::spawn(move || {
        let x = 42_i32; // на стеке потока 2
        tx.send(&x as *const i32 as usize).unwrap();
        thread::sleep(std::time::Duration::from_millis(100));
        println!("поток 2 видит: {}", x);
    });

    let addr = rx.recv().unwrap();
    unsafe {
        let p = addr as *const i32;
        println!("поток 1 прочитал чужой стек: {}", *p); // 42
    }
    t.join().unwrap();
}
