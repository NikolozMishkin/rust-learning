struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

//two rquirement for a function to be considered as a method:
//      - must be inside an implementation block
//      - first parameter must be self

//three forms of self a method could take
//      - first form: an immyutable reference to self (&self)
//      - second form: a mutable reference to self (&mut self)
//      - third form: am ownership of self (self)
impl Car {
    //      - first form: an immyutable reference to self (&self)
    fn display_car_info(&self) {
        println!(
            "Owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }

    //      - second form: a mutable reference to self (&mut self)
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    //      - third form: am ownership of self (self)
    fn sell(self) -> Self {
        //refers to the implementating type
        self
    }

    //associated functions
    fn monthly_insurence() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurence()
    }

    //associated function: new (constructor)
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year,
            fuel_level: 0.0,
            price: 0,
        }
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };
    my_car.display_car_info();

    my_car.refuel(10.5);
    let new_owner = my_car.sell();
    // my_car.refuel(10.5);

    let new_car = Car::new(String::from("XYZ"), 2020);
}
