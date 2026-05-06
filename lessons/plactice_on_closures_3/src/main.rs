fn main() {
    let mut counter = 0;

    let mut increment_counter = || counter += 1; /* Complete the Closure definition */
    increment_counter();
    increment_counter();

    println!("Final Counter: {}", counter);
}
