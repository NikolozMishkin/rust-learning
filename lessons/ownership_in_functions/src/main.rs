use std::vec;

fn takes_ownership(v: Vec<i32>) {
    println!("Vector: {:?}", v);
}

fn gives_ovnership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn takes_and_gives_back(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("In function, var is: {var}");
}
fn main() {
    // functions that take ownership
    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1.clone()); // sanme as: vec = vec_1
    println!("vec_1: {:?}", vec_1);

    //function that give ownership back
    let vec_2 = gives_ovnership();
    println!("vec_2: {:?}", vec_2);

    //functions that take and return ownership back
    let vec_3 = takes_and_gives_back(vec_2);
    // println!("vec_2: {:?}", vec_2);
    println!("vec_3: {:?}", vec_3);

    //functions that take stack data
    let x = 10;
    stack_function(x);
    println!("In main, x is: {x}");
}
