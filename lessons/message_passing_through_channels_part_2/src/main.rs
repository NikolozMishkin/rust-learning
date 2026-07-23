use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let x = "some_value".to_string();
        println!("Sending value {x}");
        // thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });

    // rx.recv().unwrap();
    // println!("I am blocked");

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is {:?}", received_value);
                received_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
