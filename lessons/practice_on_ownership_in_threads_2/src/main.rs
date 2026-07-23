use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let x = 5;
    let v_clone = v.clone();
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v_clone);
        println!("Here's a variable : {:?}", x);
    });

    println!("The variable x is still alive {}", x);
    println!("The variable v is currenlty not alive {:?}", v);
    println!("Make approperiate changes so that it remains alive on this line");
    handle.join();
}
