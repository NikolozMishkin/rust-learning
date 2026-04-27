trait Vehicle {
    fn start_engine(&self) -> String;
    fn drive(&self) -> String;
}

struct Car;

struct Bicycle;

impl Vehicle for Car {
    fn start_engine(&self) -> String {
        "🚗 Engine started".to_string()
    }

    fn drive(&self) -> String {
        "🚗 Driving the car".to_string()
    }
}

impl Vehicle for Bicycle {
    fn start_engine(&self) -> String {
        "🚲 No engine to start for a bicycle".to_string()
    }

    fn drive(&self) -> String {
        "🚲 Pedaling the bicycle".to_string()
    }
}

fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "car" => Box::new(Car),
        "bicycle" => Box::new(Bicycle),
        _ => panic!("No vehicle of that type available"),
    }
}

fn operate_vehicle(driver: &dyn Vehicle) {
    println!("{}", driver.start_engine());
    println!("{}", driver.drive());
}

fn main() {
    let vehicle_1 = Car;
    let vehicle_2 = Car;
    let vehicle_3 = Bicycle;

    let vehicles: Vec<&dyn Vehicle> = vec![&vehicle_1, &vehicle_2, &vehicle_3]; // Fix this line

    for v in vehicles {
        operate_vehicle(v);
    }
}
