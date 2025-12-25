fn main() {
    let n: i32 = 5;
    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    /* Complete the code after this line */
    let mut f = 0;
    let mut b = 0;

    while f != n {
        f = f + 1;
        square_of_sum = f + square_of_sum;
        b = f * f;
        sum_of_squares = b + sum_of_squares;
    }
    square_of_sum = square_of_sum * square_of_sum;
    let difference = square_of_sum - sum_of_squares;
    println!(
        "n = {} and the difference of the square_of_sum and sum_of_squares is {}",
        n, difference
    );
}
