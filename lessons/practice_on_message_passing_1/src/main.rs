use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let my_vec = vec![1, 2, 3, 4, 5];
        for i in my_vec {
            // sleep(Duration::from_secs(5));
            tx.send(i).unwrap();
        }
    });

    thread::spawn(move || {
        let my_vec = vec![6, 7, 8, 9, 10];
        for i in my_vec {
            // sleep(Duration::from_secs(5));
            tx_clone.send(i).unwrap(); // fix this line
        }
    });

    for recieved_vals in rx {
        println!("I recieved the value of {}", recieved_vals);
    }
}
