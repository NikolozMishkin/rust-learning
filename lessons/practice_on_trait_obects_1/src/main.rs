trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

struct Lion {
    name: String,
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Roar!");
    }
}

struct Penguin {
    name: String,
}

impl Animal for Penguin {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Honk!");
    }
}

fn describe_animal(animal: Box<dyn Animal>) {
    // This line needs a fix
    println!("The {} says:", animal.name());
    animal.make_sound();
}

fn main() {
    let lion = Lion {
        name: "Simba".to_string(),
    };
    let penguin = Penguin {
        name: "Happy Feet".to_string(),
    };

    // The calls to function needs fixes
    describe_animal(Box::new(lion));
    describe_animal(Box::new(penguin));
}
