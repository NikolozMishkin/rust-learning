trait Sound {
    fn animal_sound(&self) -> String {
        "I dont have sound!".to_string()
    }
}
struct Dog;
struct Cat;
struct Fish;

impl Sound for Dog {
    fn animal_sound(&self) -> String {
        "woof".to_string()
    }
}
impl Sound for Cat {
    fn animal_sound(&self) -> String {
        "meow".to_string()
    }
}
impl Sound for Fish {}
fn main() {
    let dog = Dog;
    let cat = Cat;
    let fish = Fish;
    println!("Dog Sound: {}", dog.animal_sound());
    println!("Cat Sound: {}", cat.animal_sound());
    println!("Fish Sound: {}", fish.animal_sound());
}
