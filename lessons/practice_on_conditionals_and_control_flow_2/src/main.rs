fn main() {
    let n: u32 = 20;

    /* Add your code below this line */
    let mut sum_of_3 = 0;
    let mut sum_of_5 = 0;

    for i in (0..=n).step_by(3) {
        sum_of_3 = sum_of_3 + i;
    }

    for f in (0..=n).step_by(5) {
        sum_of_5 = sum_of_5 + f;
    }

    let total_sum = sum_of_3 + sum_of_5;
    println!(
        "The sum of all multiples of 3 or 5 up to {} is {}",
        n, total_sum
    );
}
