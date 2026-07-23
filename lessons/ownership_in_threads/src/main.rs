// -------------------------------------------
// 	Ownership and Threads
//              - Prerequiste: Closures
// -------------------------------------------

// struct ____my_closure {
//     x: String,
// }

// impl Fn for ____my_closure {
//     extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//         println!("thread spawned");
//         println!("{}", self.x);
//         println!("thread ended");
//     }
// }

use std::thread;
fn main() {
    let x = "some string".to_string();
    let my_closure = move || {
        println!("thread spawned");
        // let y = x;
        println!("{x}");
        println!("thread ended");
    };

    let thread = thread::spawn(my_closure);
    println!("after thread spawn");
    thread.join().unwrap();
    //println!("{x}");
}
