use std::collections::HashMap;
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("Student with ID {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn main() {
    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Jonny"),
        grade: String::from("100"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Sam"),
        grade: String::from("90"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    // Retrieve and print student information
    if let Some(student) = manager.get_student(1) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
    if let Some(student) = manager.get_student(2) {
        println!("Student ID: {}", student.id);
        println!("Student Name: {}", student.name);
        println!("Student Grade: {}", student.grade);
    }
}
