fn my_fn(s: &str) {
    print!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    print!("Co,mputing multiplication:");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
//expression versus statement
//expressins return a valid value
//statrements returns a unit value
fn main() {
    my_fn("This is my function");
    let str = "Function call with a variable ";
    my_fn(str);
    let answer = multiplication(10, 15);
    let result = basic_math(10, 15);
    let (multiplication, addition, subtraction) = basic_math(10, 15);

    //code blocks

    let full_name = {
        let first_name = "John";
        let last_name = "Doel";
        println!("{answer}");
        format!("{first_name} {last_name}")
    };
    // print!("{first_name}");
}
