use std::collections::HashMap;

struct Student {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(
    student_database: &mut HashMap<i32, Student>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    let student = Student { name, age, grade };
    if student_database.contains_key(&id) {
        println!("The id already exist");
    } else {
        student_database.insert(id, student);
    }
}

fn main() {
    let mut student_database: HashMap<i32, Student> = HashMap::new();
    add_student(
        &mut student_database,
        1,
        String::from("Mike"),
        16,
        String::from("80"),
    );
    add_student(
        &mut student_database,
        2,
        String::from("Ammy"),
        17,
        String::from("75"),
    );

    // Printing the student database
    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}
