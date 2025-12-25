fn main() {
    //arithmetic operators
    // +, -, *, /, %
    // note: ^ is not for exponential
    // pow() method is used for exponential
    println!("remainder after division: {}", 10 % 3);

    //comparison operators
    //equality: == (not to be confused with assignment operator , i.e. =)
    //ineqyuality: !=
    //relational: >, <, >=, <=
    let a = 10;
    let b = 20;
    print!(
        "a == b: {}, a != b: {}, a > b: {}, a < b: {}, a >= b: {}, a <= b: {}",
        a == b,
        a != b,
        a > b,
        a < b,
        a >= b,
        a <= b
    );

    //logical operators
    //AND: &&
    //OR: ||
    //NOT: !

    let a = 10;
    let b = 20;
    if (a > 5) && (b < 25) {
        println!("\nCondition satisfied");
    } else {
        println!("\nCondition not satisfied");
    }

    //assignment operators
    // add and assign: +=
    //subtract and assign: -=
    //multiply and assign: *=
    //divide and assign: /=
    //remainder and assign: %=

    let mut x = 5;
    x += 10; // => x = x + 5;
    x -= 10; // => x = x - 10;
    x *= 10; // => x = x * 10;
    x /= 10; // => x = x / 10;
    x %= 10; // => x = x % 10;

    //bitwise operators
    //only work on integer types
    //AND: & //sets bit to 1 if both bits are 1
    //OR: | // stets bit to 1 if at least one of the bits is 1
    //XOR: ^  // sets bit to 1 if only one of the bits is 1
    //NOT: ~ // invets all bits form 0 to 1 and 1 to 0
    //left shift: << // shifts bits to the left, adding 0s on the right
    //right shift: >> // shifts bits to the right, adding 0s on the left

    let x: u8 = 4;
    println!("{}", x & x); // => 0000_0100 & 0000_0100 = 0000_0100
    let y: u8 = 4 << 1; // => 0000_0100 -> 0000_1000
}
