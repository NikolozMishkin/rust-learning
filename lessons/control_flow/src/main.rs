use std::vec;

fn main() {
    //loops
    'outer: loop {
        loop {
            println!("Simple loop");
            break 'outer;
        }
    }

    let a = loop {
        break 5;
    };

    //for loops
    let vec = vec![45, 30, 85, 90, 41, 39];
    for i in vec {
        println!("{i}");
    }

    //compound data types versus collections
    //conmound data types: fixed-size, known at compile time(eg. tuples, arrays)
    //collections: variable-size, can grow or shrink at runtime(eg. vectors, strings, hashmaps)

    //while loop
    let mut num = 0;
    while num < 10 {
        num = num + 1;
    }
}
