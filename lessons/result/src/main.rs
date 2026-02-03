struct Student {
    name: String,
    grade: Option<u32>,
}

// no mechanism: if student exists in the database
//reasons of returning None: 1. student does not exist in data base
//                           2. grades not available

//exact reason of None: Unknown
//from context: grades not available for a valid student

//enum Result <T, E>{
//    Ok (T),
//    Err (E),
//}

//check if student exist in database
// fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
//     for student in student_db {
//         if student.name == *student_name {
//             return Ok(());
//         }
//     }
//     Err(String::from("Student not found"))
// }

// fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
//     for student in student_db {
//         if student.name == *student_name {
//             return student.grade;
//         }
//     }
//     None
// }

fn check_student_get_grade(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(95),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];

    let student_name = String::from("Adam");
    let check_student = check_student_get_grade(&student_name, &student_db);

    match check_student {
        Ok(option_grade) => {
            // let student_grade = get_grade(&student_name, &student_db);
            if let Some(grade) = option_grade {
                println!("Grade is {grade}");
            }
        }
        Err(error_msg) => println!("{}", error_msg),
    }

    // match student_grade {
    //     Some(grade) => println!("Grade is {grade}"),
    //     None => {}
    // }

    // if let Some(grade) = student_grade {
    //     println!("Grade is {grade}");
    // }
}

//enum Option<T> {
//    None,
//    Some(T),
//}
