use std::thread;

fn main() {
    let mut thread_vec = vec![];
    for i in 0..10 {
        thread_vec.push(
            thread::spawn(|| {
                println!("Hi from Thread_1");
            }),
            // insert code here
            // Spawn a thread
            // include a simple print statement with a msg of "Hi from Thread"
        );
    }

    // The code below will make sure that all the threads go to completion
    for i in thread_vec {
        i.join();
    }
}
