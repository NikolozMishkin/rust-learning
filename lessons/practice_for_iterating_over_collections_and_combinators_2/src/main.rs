use std::result;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = 0;

    /* The code in the for loop needs to be replaced */
    for &num in &numbers {
        if num % 2 != 0 {
            let squared_num = num * num;
            result += squared_num;
        }
    }

    let result: i32 = numbers
        .iter()
        .filter(|&&num| num % 2 != 0)
        .map(|&num| num * num)
        .sum();

    println!("Result without combinators: {}", result);
}
