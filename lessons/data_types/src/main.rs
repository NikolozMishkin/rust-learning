fn main() {
    //unsigned integers
    let unsigned_num: u8 = 5; // u16, u32, u64, u128

    //signed integers
    let signed_num: i8 = 5; // i16, i32, i64, i128

    //floating point numbers
    let float_num: f32 = 5.0; // f64

    //platform specific numbers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    //characters
    let char: char = 'a';

    //boolean
    let b: bool = true;

    //type aliasing
    type Age = u8;
    let peter_age: Age = 42;

    //type conversion
    let a: i32 = 10;
    let b: f64 = a as f64;
}
