fn main() {
    //strings
    let fuxe_string: &'static str = "Fixed length string ";
    let mut flexible_str: String = String::from("This stribg will grow");
    flexible_str.push('s');

    //arrays
    let mut array_1: [i32; 5] = [4, 5, 6, 9, 8];
    let num = array_1[3];
    println!("{:?}", array_1);
    let array_2 = [0; 10];

    //vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 7, 8, 9];
    let num = vec_1[3];

    //tuples
    let my_info = ("Salary", 40000, "Age", 40);
    let salary_value = my_info.1;
    let (salary, salary_value, age, age_value) = my_info;

    //empty tuple
    let () = ();
}
