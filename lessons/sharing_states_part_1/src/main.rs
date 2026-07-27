use std::{sync::Mutex, thread};
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    let lock_m = m.lock().unwrap();
    println!("m is: {:?}", *lock_m);
    drop(lock_m); // add later on

    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked");
}
