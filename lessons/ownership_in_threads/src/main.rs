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

fn my_super_fn() -> i32 {
    println!("thread spawned");
    println!("thread ended");

    7
}

use std::thread;
fn main() {
    let x = "some string".to_string();
    let y = "some string".to_string();
    let foo = false;
    let my_closure = move || {
        println!("thread spawned");
        // let y = x;
        println!("{x}");
        println!("{y}");
        println!("{foo}");
        println!("thread ended");
        6
    };
    let x = "some string".to_string();
    (move || {
        println!("thread spawned");
        // let y = x;
        println!("{x}");
        println!("thread ended");
    })();
    let x = "some string".to_string();
    let thread = thread::spawn(my_closure);
    let thread = thread::spawn(my_super_fn);
    println!("after thread spawn");
    let resp = thread.join().unwrap();
    println!("after thread join, resp: {resp}");
    //println!("{x}");
}
