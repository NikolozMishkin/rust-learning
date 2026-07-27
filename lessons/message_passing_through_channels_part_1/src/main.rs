// Example 1
/*
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    // let rx_clone = rx.clone();
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("Sending value {i}");
            tx_clone.send(i).unwrap();
        });
    }

    drop(tx);
    let recv_val = rx.recv().unwrap();
    println!("Recieved {recv_val}");
    let recv_val = rx.recv().unwrap();
    println!("Received {recv_val}");

    // for message in rx {
    //     println!("Received {message}");
    // }
}
*/

// Example 2
mod complex_while;

use std::{sync::mpsc, thread};
fn main() {
    complex_while::demo();

    let (tx, rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let mut i = "5".to_string();
        println!("Sending value {i}");
        tx.send(i.clone()).unwrap();
        tx.send(i).unwrap();
    });

    // let received_val = rx.recv().unwrap();
    println!("just run");

    // t.join();
    // println!("just ended");
    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is: {received_value}");
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
