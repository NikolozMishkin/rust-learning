pub trait VehicleHorn {
    fn horn_sound(&self) -> String {
        "peep peep".to_string()
    }
}

struct Car {}

struct Truck {}

impl VehicleHorn for Car {}
impl VehicleHorn for Truck {}

fn compare_horn_sound(vehicle_1: impl VehicleHorn, vehicle_2: impl VehicleHorn) -> bool {
    // complete the function definition
    vehicle_1.horn_sound() == vehicle_2.horn_sound()
}

fn main() {
    let car = Car {};
    let truck = Truck {};
    assert_eq!(compare_horn_sound(car, truck), true);
}
