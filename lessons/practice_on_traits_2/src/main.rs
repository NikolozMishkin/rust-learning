trait Vehicle {
    fn speed(&self) -> f64 {
        0.0
    }
}

struct Car {
    model: String,
    speed: f64,
}

impl Vehicle for Car {
    fn speed(&self) -> f64 {
        self.speed
    }
}

struct Bicycle {
    brand: String,
}

impl Vehicle for Bicycle {}
struct Bus {
    model: String,
    speed: f64,
}

impl Vehicle for Bus {
    fn speed(&self) -> f64 {
        self.speed
    }
}
fn main() {
    let car = Car {
        model: "Camry".to_string(),
        speed: 120.0,
    };
    let bicycle = Bicycle {
        brand: "MountainBike".to_string(),
    };
    let bus = Bus {
        model: "BMC".to_string(),
        speed: 100.0,
    };

    car.speed();
    bicycle.speed();
    bus.speed();
}
