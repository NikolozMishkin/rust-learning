struct Employee {
    name: String,
    salary: u32,
    department: String,
}

fn process_employees<V1, V2>(
    employees: Vec<Employee>,
    name_transformer: V1,
    salary_filter: V2,
) -> Vec<String>
where
    V1: Fn(&str) -> String,
    V2: Fn(u32) -> bool,
{
    let mut processed_names = Vec::new();

    for employee in employees {
        if salary_filter(employee.salary) {
            processed_names.push(name_transformer(&employee.name));
        }
    }

    processed_names
}

fn main() {
    let employees = vec![
        Employee {
            name: String::from("Alice"),
            salary: 60000,
            department: String::from("Engineering"),
        },
        Employee {
            name: String::from("Bob"),
            salary: 75000,
            department: String::from("Sales"),
        },
        Employee {
            name: String::from("Charlie"),
            salary: 50000,
            department: String::from("Marketing"),
        },
    ];

    let transform_name_to_uppercase = |name: &str| name.to_uppercase();

    let filter_salary_above_threshold = |salary: u32| salary > 60000;

    let processed_names = process_employees(
        employees,
        transform_name_to_uppercase,
        filter_salary_above_threshold,
    );

    println!("Processed names: {:?}", processed_names);
}
