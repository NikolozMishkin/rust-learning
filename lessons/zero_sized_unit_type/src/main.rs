// fn f1() {}
fn f1() -> () {}
fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err(String::from("Error: Division by zero")),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}
fn main() {
    let x = ();
    let y = f1();

    let z = println!("Hello, world!");

    let mut vec: Vec<()> = Vec::with_capacity(0);
    vec.push(());
    vec.push(());
    vec.push(());
    assert_eq!(3, vec.len());
    println!("{}", vec.capacity());

    /*
    Unit Type                          || Never Types
    1. No meaningful value             || 1. Never produces a value
    2. Function returning unit         || 2. Function returning never, will never
    always returns normally            || returns normally
    3. single value, which can not be  || 3. No associated value, and can be coerced
    coerced                            || to all types.
    */
}
