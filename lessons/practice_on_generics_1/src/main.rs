enum Operation<T> {
    Addition(T, T),
    Subtraction(T, T),
    Multiplication(T, T),
    Division(T, T),
}

fn main() {
    let op_1 = Operation::Addition(5, 10);
    let op_2 = Operation::Multiplication(3.5, 2.0);
    let op_3 = Operation::Subtraction(3.5, 2.0);
    let op_4 = Operation::Division(2, 3);
}
