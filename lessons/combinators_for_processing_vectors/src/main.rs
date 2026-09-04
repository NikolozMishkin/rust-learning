fn main() {
    // 1. skip
    let csv_data = vec!["name,age", "Alice,30", "Bob,25"];
    let data_without_headers = csv_data.into_iter().skip(1).collect::<Vec<&str>>();
    println!("First row removed: {:?}", data_without_headers);

    // 2. take: Previewing data
    let data = vec![
        String::from("row1"),
        String::from("row2"),
        String::from("row3"),
        String::from("row4"),
    ];
    let preview = data.iter().take(2).collect::<Vec<&String>>();
    println!("Data preview: {:?}", preview);

    // 3. Enumerate: Reporting or Logging Line Numbers
    let code = vec!["fn main() {", " println!(\"Hello\")", "}"];
    let code_with_line_numbers = code.into_iter().enumerate().collect::<Vec<(usize, &str)>>();
    println!("Code with line numbers {:?}", code_with_line_numbers);

    // Accumulators
    // 4. fold
    let nums = vec![1, 2, 3, 4];
    let sum_of_squares = nums.iter().fold(0, |acc, &x| acc + x * x);
    println!("Sum of squares of 1,2,3,4 = {sum_of_squares}");

    // 5. reduce
    let nums = vec![1, 2, 3, 4];
    let result = nums.into_iter().reduce(|a, b| a.max(b));
    println!("{:?}", result);
    // Accumulates: (((1.max(2)).max(3)).max(4))
}
