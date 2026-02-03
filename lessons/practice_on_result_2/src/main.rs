fn calculate_square(num: i32) -> Result<i32, String> {
    if num >= 0 {
        let result = num * num;
        println!("The square of {} is: {}", num, result);
        Ok(result)
    } else {
        Err("Negative number provided".to_string())
    }
}

fn main() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }
}
