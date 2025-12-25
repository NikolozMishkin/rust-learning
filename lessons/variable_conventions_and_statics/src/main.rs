fn main() {
    //variable conventions and unused variables
    let _number_one = 45;
    let _X = 40_000;

    //statics
    static WELCOME: &str = "Welcome to Rust";
    const PI: f32 = 3.14;
    //const do not take space in memory
    let a: f32 = PI;
    let b: f32 = PI;
    //statict take space in memory
    let y = WELCOME;
    let z = WELCOME;
}
