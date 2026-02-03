#[derive(Debug)]
enum Value {
    // Add code here
    // Define two variants of Integer and Float (with associated int and float types respectively)
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
}
