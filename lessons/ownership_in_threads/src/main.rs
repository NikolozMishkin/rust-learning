// -------------------------------------------
// 	Ownership and Threads
//              - Prerequiste: Closures
// -------------------------------------------

use std::thread;
fn main() {
    let x = "some string".to_string();

    thread::spawn(move || {
        // let y = x;
        println!("{x}");
    });
    //println!("{x}");
}
