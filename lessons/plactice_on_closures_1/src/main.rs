fn main() {
    let x = 10;
    let add_to_x = |y| x + y; /* Add closure definition here */

    let result = add_to_x(5);
    println!("Result: {}", result);
}
