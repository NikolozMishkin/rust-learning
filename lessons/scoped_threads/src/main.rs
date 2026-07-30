use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3];

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope");
            println!("vec: {:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Another Thread inside scope");
            //vec.push(4);
            println!("vec: {:?}", vec);
        });
    });

    println!("The scope finished");
    vec.push(5);
    println!("vec: {:?}", vec);
}
