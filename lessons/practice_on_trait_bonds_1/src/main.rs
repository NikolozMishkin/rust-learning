trait Greeting {
    fn greet(&self) -> String {
        "Hello from Rust!".to_string()
    }
}

fn print_greeting1<T: Greeting>(input: &T) {
    // Fix using trait bound
    println!("{}", input.greet());
}

fn print_greeting2(input: &impl Greeting) {
    // Fix using impl trait syntax
    println!("{}", input.greet());
}

fn print_greeting3<T>(input: &T)
// Fix by using the where clause
where
    T: Greeting,
{
    println!("{}", input.greet());
}

struct Greeter;

impl Greeting for Greeter {}

fn main() {
    let greeter_instance = Greeter;

    print_greeting1(&greeter_instance);
    print_greeting2(&greeter_instance);
    print_greeting3(&greeter_instance);
}
