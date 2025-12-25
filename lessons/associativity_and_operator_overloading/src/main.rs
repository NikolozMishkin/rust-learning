use std::backtrace;

fn main() {
    //associativity

    let x = 8 / 4 / 2; // left to right associativity (8 / 4) / 2 = 1
    let mut y = 42;
    // x = y = 0; // (y = 0), x = (y = 0) // x = ()

    //rigth to left
    // = , +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=

    let mut a = 1;
    let mut b = 2;
    let mut c = 3;
    // b += c += 42;

    //explicit boolean in conditionals
    let x = 0;
    if x != 0 {}

    //operator overloading
    let a = 10 + 20;
    let b = String::from("1") + "2";
    println!("a: {}, b: {}", a, b);
}
