#[derive(Debug)]
struct CarEngine {
    model: String,
    horsepower: u32,
}
#[derive(Debug)]
struct GasolineCar {}

#[derive(Debug)]
struct ElectricEngine {
    model: String,
    power: u32,
}

#[derive(Debug)]
struct ElectricCar {}

trait Vehicle {
    type EngineType;
    fn get_engine(&self) -> Self::EngineType;
}

impl Vehicle for GasolineCar {
    type EngineType = CarEngine; // Fix this line
    fn get_engine(&self) -> Self::EngineType {
        CarEngine {
            model: "V8".to_string(),
            horsepower: 400,
        }
    }
}

impl Vehicle for ElectricCar {
    type EngineType = ElectricEngine; // Fix this line
    fn get_engine(&self) -> Self::EngineType {
        ElectricEngine {
            model: "Electric Motor".to_string(),
            power: 300,
        }
    }
}

fn main() {
    let gasoline_car = GasolineCar {};
    let electric_car = ElectricCar {};

    println!("Gasoline Car engine: {:?}", gasoline_car.get_engine());
    println!("Electric Car engine: {:?}", electric_car.get_engine());
}
