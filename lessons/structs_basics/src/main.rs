//struct = group related data

//name-field struct

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    privce: u32,
}

fn print_tupl_coordinats(coords: Point_2D) {
    println!("X: {}, Y: {}", coords.0, coords.1);
}

struct Point_2D(i32, i32);

struct Point_3D(i32, i32, i32);

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        privce: 5_000,
    };
    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    let extracted_owner = my_car.owner.clone();
    println!("Owner: {}", my_car.owner); //partial move: some portion of the data is moved out of struct instance 

    let another_car = Car {
        owner: String::from("new_name"),
        ..my_car
    };

    // println!("Another car owner: {}", my_car.owner);

    //tuple structs
    let point_2D = (1, 3);
    let point_3D = (4, 10, 13);

    let point1 = Point_2D(1, 3);
    let point2 = Point_3D(4, 10, 13);

    //unit-like struct
    struct ABC;
}
