fn invoker(operation: fn(i32) -> i32, operand: i32) -> i32 {
    operation(operand)
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let square = |x: i32| x * x;

    let result = invoker(square, 4);
    println!("Result is: {}", result);
}
