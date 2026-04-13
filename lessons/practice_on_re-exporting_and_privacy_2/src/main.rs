mod University {
    pub struct Student {
        pub name: String,
        pub marks: u8,
        pub grade: char,
    }
}

use University::Student;

fn main() {
    let mut student_1 = Student {
        name: String::from("Alice"),
        marks: 75,
        grade: 'A',
    };
    println!("{} got {} grade", student_1.name, student_1.grade);
}
