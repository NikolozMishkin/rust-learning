use initializing_struct_instances::Student;

fn main() {
    let std_1 = Student::new("Joseph123".to_string()).unwrap_or_default();
    println!("{:?}", std_1);

    let std_2 = Student::default();
    println!("{:?}", std_2);

    let std_3 = Student {
        age: 12,
        ..Default::default()
    };
}
